use actix_web::{error::ErrorNotFound, get, services, web, Responder};
use sea_orm::DatabaseConnection;
use serde_json::json;

use crate::{dbaccess::dbblog, error::ServerResult, model::BlogQuery};

pub fn blog_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(services![
        get_bolg_search_result,
        get_blog_list_item,
        get_one_blog,
    ]);
}

#[get("/search")]
pub async fn get_bolg_search_result(
    params: web::Query<BlogQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let query = params.into_inner();

    let res = dbblog::blog_search(&query, &conn, true).await?;

    Ok(web::Json(res))
}

#[get("/{blog_id}")]
pub async fn get_one_blog(
    path: web::Path<i32>,
    params: web::Query<BlogQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let params = params.into_inner();
    let blog_id = path.into_inner();
    let res = dbblog::get_one_by_id(blog_id, params, &conn)
        .await
        .map_err(|err| ErrorNotFound(err.to_string()))?;

    Ok(web::Json(json!({"msg":"请求成功","data":res})))
}

#[get("/{page_size}/{index}")]
pub async fn get_blog_list_item(
    path: web::Path<(usize, usize)>,
    params: web::Query<BlogQuery>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let path = path.into_inner();
    let query = params.into_inner();

    let (data, pages, items) = dbblog::get_item_list_with_query(&query, path, &conn, true).await?;

    Ok(web::Json(json!({
        "msg":"获取列表成功",
        "data":{
            "items":items,
            "pages":pages,
            "list": data
        }
    })))
}
