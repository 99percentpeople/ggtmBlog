use super::*;
use crate::{error::ServerResult, Settings};
use actix_files::NamedFile;
use actix_identity::Identity;
use actix_multipart::{Field, Multipart};
use entity::{file, user};
use futures::{try_join, TryStreamExt};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, EntityTrait, ModelTrait, PaginatorTrait, Set,
};
use serde_json::json;
use std::{
    io::ErrorKind,
    ops::Not,
    path::{self, Path, PathBuf},
    sync::Arc,
};
use tokio::{fs, io::AsyncWriteExt, sync::Mutex};
use uuid::Uuid;

pub fn files_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(services![
        upload_file,
        get_file,
        delete_file,
        get_files_for_user,
        put_file
    ]);
}

pub async fn update_file(
    field: &mut Field,
    filepath: &path::Path,
) -> ServerResult<(String, String)> {
    let content_type = field.content_disposition();

    let original_filename = content_type
        .get_filename()
        .ok_or(ErrorBadRequest("需要文件名称"))?
        .to_owned();

    let file_extesion = Path::new(&original_filename)
        .extension()
        .ok_or_else(|| ErrorBadRequest(format!("文件名不合法: {}", original_filename)))?
        .to_string_lossy();

    let filename = format!("{}.{}", Uuid::new_v4().to_string(), file_extesion);

    let filepath: path::PathBuf = [filepath, path::Path::new(&filename)].into_iter().collect();

    log::debug!("filepath = {filepath:?}");
    // File::create is blocking operation, use threadpool
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filepath)
        .await
        .map_err(ErrorInternalServerError)?;

    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.try_next().await? {
        f.write_all(&chunk)
            .await
            .map_err(ErrorInternalServerError)?;
    }
    Ok((original_filename, filename))
}

#[post("")]
async fn upload_file(
    id: Identity,
    payload: Multipart,
    conn: web::Data<DatabaseConnection>,
    settings: web::Data<Settings>,
) -> ServerResult<impl Responder> {
    let user_info = try_verify(&id, 3)?;
    let base_path = settings.application.file_url.to_owned();
    let user_dir = format!("{}-{}", user_info.id, user_info.username);

    let stream = payload.map_err(ErrorBadRequest).map_ok(|mut field| {
        let base_path = base_path.to_owned();
        let user_dir = user_dir.to_owned();
        let conn = conn.to_owned();
        let path = [&base_path, &user_dir].iter().collect::<path::PathBuf>();
        async move {
            path.exists()
                .not()
                .then(|| std::fs::create_dir_all(&path))
                .unwrap_or(Ok(()))
                .map_err(ErrorInternalServerError)?;

            let (original_filename, filename) = update_file(&mut field, &path.as_path()).await?;

            let path = [&user_dir, &filename]
                .iter()
                .collect::<PathBuf>()
                .to_string_lossy()
                .into_owned();

            let file_type = field.content_type().to_string();

            let model = file::ActiveModel {
                id: NotSet,
                path: Set(path),
                file_type: Set(Some(file_type)),
                file_name: Set(Some(original_filename)),
                user_id: Set(user_info.id),
            }
            .insert(conn.get_ref())
            .await?;

            <ServerResult<_>>::Ok(model)
        }
    });

    let models = Arc::new(Mutex::new(Vec::new()));
    let nums = std::thread::available_parallelism()
        .unwrap_or(std::num::NonZeroUsize::new(1).unwrap())
        .get();
    log::debug!("thread nums = {nums}");
    
    stream
        .try_for_each_concurrent(nums, |handle| {
            let models = Arc::clone(&models);
            async move {
                let model = handle.await?;
                models.lock().await.push(model);
                Ok(())
            }
        })
        .await?;

    Ok(web::Json(
        json!({"msg":"上传成功","data": *models.lock().await}),
    ))
}

#[get("/{file_id}")]
pub async fn get_file(
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
    settings: web::Data<Settings>,
) -> ServerResult<impl Responder> {
    let base_path = &settings.application.file_url;
    let file_id = path.into_inner();
    let file = file::Entity::find_by_id(file_id)
        .one(conn.get_ref())
        .await?
        .ok_or(ErrorNotFound(format!("{file_id}未找到")))?;

    let path: path::PathBuf = [base_path, &file.path].iter().collect();

    let mut parameters = Vec::new();

    if let Some(name) = Path::new(&file.path)
        .file_stem()
        .map(|f| f.to_string_lossy().into_owned())
    {
        parameters.push(header::DispositionParam::Name(name));
    }

    if let Some(file_name) = file.file_name {
        parameters.push(header::DispositionParam::Filename(file_name));
    }
    Ok(NamedFile::open_async(path)
        .await?
        .set_content_disposition(header::ContentDisposition {
            disposition: header::DispositionType::Attachment,
            parameters,
        }))
}

#[delete("/{file_id}")]
pub async fn delete_file(
    id: Identity,
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
    settings: web::Data<Settings>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let base_path = &settings.application.file_url;
    let file_id = path.into_inner();
    let old = file::Entity::find_by_id(file_id)
        .one(conn.get_ref())
        .await?
        .ok_or(ErrorNotFound(format!("{file_id}未找到")))?;

    let path: path::PathBuf = [&base_path, &old.path].iter().collect();

    match fs::remove_file(path).await {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
        Err(e) => Err(ErrorInternalServerError(e)),
    }?;

    file::ActiveModel::from(old).delete(conn.get_ref()).await?;

    Ok(web::Json(json!({"msg":"删除成功"})))
}

#[get("/info/{page_size}/{index}")]
async fn get_files_for_user(
    id: Identity,
    path: web::Path<(u64, u64)>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let user_info = try_verify(&id, 3)?;
    let user = user::Entity::find_by_id(user_info.id)
        .one(conn.get_ref())
        .await?
        .ok_or(ErrorNotFound("未找到"))?;
    let (page_size, index) = path.into_inner();
    let pages = user
        .find_related(file::Entity)
        .paginate(conn.get_ref(), page_size);
    let (data, pages, items) = try_join!(
        pages.fetch_page(index),
        pages.num_pages(),
        pages.num_items()
    )?;

    Ok(web::Json(json!({
    "msg":"获取成功",
        "data": {
            "list" : data,
            "pages" : pages,
            "items" : items
        }
    })))
}

#[put("/{file_id}")]
async fn put_file(
    id: Identity,
    path: web::Path<i32>,
    mut payload: Multipart,
    conn: web::Data<DatabaseConnection>,
    settings: web::Data<Settings>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let file_id = path.into_inner();

    let file = file::Entity::find_by_id(file_id)
        .one(conn.get_ref())
        .await?
        .ok_or(ErrorNotFound("未找到该文件"))?;
    let base_path = settings.application.file_url.to_owned();
    let path: path::PathBuf = [&base_path, &file.path].iter().collect();
    while let Some(mut field) = payload.try_next().await? {
        if field.content_type().type_() == mime::IMAGE {
            update_file(&mut field, &path).await?;
        };
    }

    Ok(web::Json(json!({"msg":"更新成功"})))
}
