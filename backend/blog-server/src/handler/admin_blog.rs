use super::*;
use crate::dbaccess::dbblog;
use crate::model::{BlogQuery, UpdateBlog};
use crate::{error::ServerResult};
use actix_identity::Identity;

use actix_web::error::ErrorNotFound;
use actix_web::{delete, get, post, put, Responder};
use actix_web::{services, web};
use entity::blog;


use serde_json::json;

pub fn admin_blog_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(services![
        get_bolg_search_result,
        get_blog_list,
        get_blog_list_item,
        get_one_blog,
        post_blog,
        put_blog,
        delete_blog
    ]);
}

#[get("/search")]
pub async fn get_bolg_search_result(
    id: Identity,
    params: web::Query<BlogQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let conn = conn;
    let query = params.into_inner();

    let res = dbblog::blog_search(&query, &conn, false).await?;

    Ok(web::Json(res))
}

#[get("/{page_size}/{index}")]
pub async fn get_blog_list_item(
    id: Identity,
    path: web::Path<(u64, u64)>,
    params: web::Query<BlogQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let path = path.into_inner();
    let query = params.into_inner();

    let (data, pages, items) = dbblog::get_item_list_with_query(&query, path, &conn, false).await?;

    Ok(web::Json(json!({
        "msg":"获取列表成功",
        "data":{
            "items":items,
            "pages":pages,
            "list": data
        }
    })))
}

#[get("detail/{page_size}/{index}")]
pub async fn get_blog_list(
    id: Identity,
    path: web::Path<(u64, u64)>,
    params: web::Query<BlogQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let path = path.into_inner();
    let query = params.into_inner();

    let (data, pages, items) = dbblog::get_model_list_with_query(&query, path, &conn, false).await?;

    Ok(web::Json(json!({
        "msg":"获取列表成功",
        "data":{
            "items":items,
            "pages":pages,
            "list": data
        }
    })))
}

#[post("")]
async fn post_blog(
    id: Identity,
    blog: web::Json<blog::Model>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let user_info = try_verify(&id, 3)?;
    let mut blog = blog.into_inner();
    blog.user_id = user_info.id;
    let res = dbblog::insert_blog(blog, &conn).await?;

    Ok(web::Json(json!({
        "msg":"添加成功",
        "data": res
    })))
}

#[put("/{blog_id}")]
async fn put_blog(
    id: Identity,
    new_blog: web::Json<UpdateBlog>,
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let blog_id = path.into_inner();
    let new_blog = new_blog.into_inner();
    let res = dbblog::update_blog(blog_id, new_blog, &conn)
        .await
        .map_err(|err| ErrorNotFound(err.to_string()))?;

    Ok(web::Json(json!({
        "msg":"更新成功",
        "data": res
    })))
}

#[get("/{blog_id}")]
pub async fn get_one_blog(
    id: Identity,
    path: web::Path<i32>,
    params: web::Query<BlogQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let params = params.into_inner();
    let blog_id = path.into_inner();
    let res = dbblog::get_one_by_id(blog_id, params, &conn)
        .await
        .map_err(|err| ErrorNotFound(err.to_string()))?;

    Ok(web::Json(json!({"msg":"请求成功","data":res})))
}

#[delete("/{blog_id}")]
pub async fn delete_blog(
    id: Identity,
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let blog_id = path.into_inner();

    dbblog::delete_blog(blog_id, &conn).await?;

    Ok(web::Json(json!({
        "msg":"删除成功"
    })))
}
