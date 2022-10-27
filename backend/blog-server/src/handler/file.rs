use std::{
    fs::File,
    io::{ErrorKind, Write},
};

use super::*;
use crate::{error::ServerResult, Settings};
use actix_files::NamedFile;
use actix_identity::Identity;
use actix_multipart::Multipart;

use entity::{file, user};
use futures::{try_join, StreamExt, TryStreamExt};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, EntityTrait, ModelTrait, PaginatorTrait, Set,
};
use serde_json::json;
use std::path;
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
    mut payload: Multipart,
    filepath: &path::Path,
) -> ServerResult<(String, String)> {
    // iterate over multipart stream
    if let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();

        let filename = content_type
            .get_filename()
            .ok_or(ErrorBadRequest("需要文件名称"))?
            .to_owned();
        let original_filename = filename;
        let filename = format!(
            "{}{}",
            Uuid::new_v4().to_string(),
            original_filename
                .rsplit_once(".")
                .and_then(|v| Some(format!(".{}", v.1)))
                .unwrap_or_default()
        );

        let filepath: path::PathBuf = [filepath, path::Path::new(&filename)].into_iter().collect();

        log::debug!("{filepath:?}");
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| {
            std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(filepath)
        })
        .await
        .map_err(|err| ErrorInternalServerError(err))?
        .map_err(|err| ErrorInternalServerError(err))?;

        // Field in turn is stream of *Bytes* object
        while let Some(Ok(chunk)) = field.next().await {
            let data = chunk;
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || -> ServerResult<File> {
                f.write_all(&data)
                    .map_err(|err| ErrorInternalServerError(err))?;
                Ok(f)
            })
            .await
            .map_err(|err| ErrorInternalServerError(err))??
        }
        Ok((original_filename, filename))
    } else {
        Err(ErrorBadRequest("").into())
    }
}
pub async fn save_file(
    mut payload: Multipart,
    filepath: &path::Path,
) -> ServerResult<(String, String)> {
    // iterate over multipart stream
    if let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();

        let filename = content_type
            .get_filename()
            .ok_or(ErrorBadRequest("需要文件名称"))?
            .to_owned();
        let original_filename = filename;
        let filename = format!(
            "{}{}",
            Uuid::new_v4().to_string(),
            original_filename
                .rsplit_once(".")
                .and_then(|v| Some(format!(".{}", v.1)))
                .unwrap_or_default()
        );

        let filepath: path::PathBuf = [filepath, path::Path::new(&filename)].into_iter().collect();

        log::debug!("{filepath:?}");
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .map_err(|err| ErrorInternalServerError(err))?
            .map_err(|err| ErrorInternalServerError(err))?;

        // Field in turn is stream of *Bytes* object
        while let Some(Ok(chunk)) = field.next().await {
            let data = chunk;
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || -> ServerResult<File> {
                f.write_all(&data)
                    .map_err(|err| ErrorInternalServerError(err))?;
                Ok(f)
            })
            .await
            .map_err(|err| ErrorInternalServerError(err))??
        }
        Ok((original_filename, filename))
    } else {
        Err(ErrorBadRequest("").into())
    }
}

#[post("")]
async fn upload_file(
    id: Identity,
    payload: Multipart,
    conn: web::Data<DatabaseConnection>,
    settings: web::Data<Settings>,
) -> ServerResult<impl Responder> {
    let user_info = try_verify(&id, 3)?;
    let base_path = settings.application.file_path_url.to_owned();
    let user_dir = format!("{}-{}", user_info.id, user_info.username);
    let mut path: path::PathBuf = [&base_path, &user_dir].iter().collect();

    path = if !path.exists() {
        web::block(move || -> ServerResult<path::PathBuf> {
            std::fs::create_dir_all(&path)?;
            Ok(path)
        })
        .await
        .map_err(|err| ErrorInternalServerError(err))??
    } else {
        path
    };

    let (original_filename, filename) = save_file(payload, &path.as_path()).await?;
    let path: String = format!("{user_dir}/{filename}",);

    let ftype = original_filename
        .rsplit_once(".")
        .and_then(|v| Some(Some(v.1.to_string())))
        .unwrap_or(None);

    let model = file::ActiveModel {
        id: NotSet,
        path: Set(path),
        file_type: Set(ftype),
        file_name: Set(Some(original_filename)),
        user_id: Set(user_info.id),
    }
    .insert(&conn as &DatabaseConnection)
    .await?;

    Ok(web::Json(json!({"msg":"上传成功","data": model})))
}

#[get("/{file_id}")]
pub async fn get_file(
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
    settings: web::Data<Settings>,
) -> ServerResult<impl Responder> {
    let base_path = settings.application.file_path_url.to_owned();
    let file_id = path.into_inner();
    let file = file::Entity::find_by_id(file_id)
        .one(&conn as &DatabaseConnection)
        .await?
        .ok_or(ErrorNotFound(format!("{file_id}未找到")))?;

    let path: path::PathBuf = [base_path, file.path].iter().collect();

    Ok(NamedFile::open_async(path)
        .await?
        .set_content_disposition(header::ContentDisposition {
            disposition: header::DispositionType::Inline,
            parameters: file
                .file_name
                .and_then(|name| Some(vec![header::DispositionParam::Name(name)]))
                .unwrap_or(vec![]),
        }))
    // header::DispositionParam::Name(f.u.to_owned())
}

#[delete("/{file_id}")]
pub async fn delete_file(
    id: Identity,
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
    settings: web::Data<Settings>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let base_path = settings.application.file_path_url.to_owned();
    let file_id = path.into_inner();
    let old = file::Entity::find_by_id(file_id)
        .one(&conn as &DatabaseConnection)
        .await?
        .ok_or(ErrorNotFound(format!("{file_id}未找到")))?;

    let path: path::PathBuf = [&base_path, &old.path].iter().collect();
    web::block(|| std::fs::remove_file(path))
        .await
        .map_err(|err| ErrorInternalServerError(err))?
        .map_or_else(
            |e| {
                if e.kind() == ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(ErrorInternalServerError(e))
                }
            },
            |_| Ok(()),
        )?;

    file::ActiveModel::from(old)
        .delete(&conn as &DatabaseConnection)
        .await?;

    Ok(web::Json(json!({"msg":"删除成功"})))
}

#[get("/info/{page_size}/{index}")]
async fn get_files_for_user(
    id: Identity,
    path: web::Path<(usize, usize)>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let user_info = try_verify(&id, 3)?;
    let user = user::Entity::find_by_id(user_info.id)
        .one(&conn as &DatabaseConnection)
        .await?
        .ok_or(ErrorNotFound("未找到"))?;
    let (page_size, index) = path.into_inner();
    let pages = user
        .find_related(file::Entity)
        .paginate(&conn as &DatabaseConnection, page_size);
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
    payload: Multipart,
    conn: web::Data<DatabaseConnection>,
    settings: web::Data<Settings>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let file_id = path.into_inner();

    let file = file::Entity::find_by_id(file_id)
        .one(&conn as &DatabaseConnection)
        .await?
        .ok_or(ErrorNotFound("未找到该文件"))?;
    let base_path = settings.application.file_path_url.to_owned();
    let path: path::PathBuf = [&base_path, &file.path].iter().collect();

    update_file(payload, &path).await?;

    Ok(web::Json(json!({"msg":"更新成功"})))
}
