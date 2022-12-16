use super::try_verify;
use crate::dbaccess::dbtag;
use crate::error::ServerResult;
use crate::model::TagQuery;
use actix_identity::Identity;
use actix_web::error::ErrorNotFound;
use actix_web::{delete, get, post, put, Responder};
use actix_web::{services, web};
use entity::tag;
use sea_orm::DatabaseConnection;
use serde_json::json;
pub fn tag_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(services![
        get_tag_list,
        get_one_by_id,
        post_tag,
        put_tag,
        delete_tag
    ]);
}

#[get("/{page_size}/{index}")]
async fn get_tag_list(
    path: web::Path<(u64, u64)>,
    query: web::Query<TagQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let query = query.into_inner();
    let path = path.into_inner();

    let (data, pages, items) = dbtag::get_list_with_query(&query, path, &conn).await?;
    Ok(web::Json(json!({
        "msg":"获取标签列表成功",
        "data":{
            "pages":pages,
            "items":items,
            "list": data
        }
    })))
}

#[get("/{tag_id}")]
async fn get_one_by_id(
    params: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let tag_id = params.into_inner();

    let tag = dbtag::get_one_by_id(tag_id, &conn)
        .await
        .map_err(|_| ErrorNotFound("没有找到标签"))?;
    Ok(web::Json(json!({
        "msg":"获取标签成功",
        "data": tag
    })))
}

#[post("")]
async fn post_tag(
    id: Identity,
    tag: web::Json<tag::Model>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let tag = tag.into_inner();

    let res = dbtag::insert_tag(tag, &conn).await?;

    Ok(web::Json(json!({
        "msg":"添加标签成功",
        "data": res
    })))
}

#[put("/{tag_id}")]
async fn put_tag(
    id: Identity,
    new_tag: web::Json<tag::Model>,
    params: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let tag_id = params.into_inner();
    let new_tag = new_tag.into_inner();

    let res = dbtag::update_tag(tag_id, new_tag, &conn).await?;

    Ok(web::Json(json!({
        "msg":"更新标签成功",
        "data": res
    })))
}

#[delete("/{tag_id}")]
async fn delete_tag(
    id: Identity,
    params: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let tag_id = params.into_inner();

    dbtag::delete_tag(tag_id, &conn).await?;

    Ok(web::Json(json!({
        "msg":"删除标签成功"
    })))
}
