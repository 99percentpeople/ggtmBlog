use super::try_verify;
use crate::dbaccess::dbsort;
use crate::error::ServerResult;
use crate::model::SortQuery;
use actix_identity::Identity;
use actix_web::error::ErrorNotFound;
use actix_web::{delete, get, post, put, Responder};
use actix_web::{services, web};
use entity::sort;
use sea_orm::DatabaseConnection;
use serde_json::json;
pub fn sort_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(services![
        get_sort_list,
        get_one_by_id,
        post_sort,
        put_sort,
        delete_sort
    ]);
}

#[get("/{page_size}/{index}")]
async fn get_sort_list(
    path: web::Path<(usize, usize)>,
    query: web::Query<SortQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let query = query.into_inner();
    let path = path.into_inner();

    let (data, pages, items) = dbsort::get_list_with_query(&query, path, &conn).await?;
    Ok(web::Json(json!({
        "msg":"获取分类列表成功",
        "data":{
            "pages":pages,
            "items":items,
            "list": data
        }
    })))
}

#[get("/{sort_id}")]
async fn get_one_by_id(
    params: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let sort_id = params.into_inner();

    let sort = dbsort::get_one_by_id(sort_id, &conn)
        .await
        .map_err(|_| ErrorNotFound("没有找到"))?;
    Ok(web::Json(json!({
        "msg":"获取分类成功",
        "data": sort
    })))
}

#[post("")]
async fn post_sort(
    id: Identity,
    sort: web::Json<sort::Model>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let sort = sort.into_inner();

    let res = dbsort::insert_sort(sort, &conn).await?;

    Ok(web::Json(json!({
        "msg":"添加分类成功",
        "data": res
    })))
}

#[put("/{sort_id}")]
async fn put_sort(
    id: Identity,
    new_sort: web::Json<sort::Model>,
    params: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let sort_id = params.into_inner();
    let new_sort = new_sort.into_inner();

    let res = dbsort::update_sort(sort_id, new_sort, &conn).await?;

    Ok(web::Json(json!({
        "msg":"更新分类成功",
        "data": res
    })))
}

#[delete("/{sort_id}")]
async fn delete_sort(
    id: Identity,
    params: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let sort_id = params.into_inner();

    dbsort::delete_sort(sort_id, &conn).await?;

    Ok(web::Json(json!({
        "msg":"删除分类成功"
    })))
}
