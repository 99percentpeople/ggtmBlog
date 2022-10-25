use actix_identity::Identity;
use actix_web::{delete, error::ErrorBadRequest, get, post, services, web, Responder};

use entity::comment;

use regex::Regex;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet, Set};
use serde_json::json;

lazy_static! {
    static ref RE_CONTENT: Regex = Regex::new(r#"^(?:(?:\s*)\S(?:\s*)){5,200}$"#).unwrap();
    static ref RE_NICKNAME: Regex = Regex::new(r#"^[A-Za-z0-9\u4e00-\u9fa5]{3,10}$"#).unwrap();
    static ref RE_MAIL: Regex =
        Regex::new(r#"^[A-Za-z0-9\u4e00-\u9fa5]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$"#).unwrap();
    static ref RE_URL: Regex = Regex::new(
        r#"^((ht|f)tps?)://[\w\-]+(\.[\w\-]+)+([\w\-\.,@?^=%&:/~\+#]*[\w\-@?^=%&/~\+#])?$"#
    )
    .unwrap();
}

use crate::{dbaccess::dbcomment, error::ServerResult, handler::try_verify};

pub fn comment_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(services![post_comment, comments, delete_comment]);
}

#[post("/{blog_id}")]
async fn post_comment(
    comment: web::Json<comment::Model>,
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let blog_id = path.into_inner();
    let mut comment = comment.into_inner();
    /* 处理回复内容 */
    comment.content = comment.content.trim().into();
    /* 验证回复内容 */
    RE_CONTENT
        .is_match(&comment.content)
        .then(|| ())
        .ok_or(ErrorBadRequest("请输入5~200个字符,不包含空格"))?;
    RE_NICKNAME
        .is_match(&comment.nickname)
        .then(|| ())
        .ok_or(ErrorBadRequest("昵称不合法"))?;
    RE_MAIL
        .is_match(&comment.email)
        .then(|| ())
        .ok_or(ErrorBadRequest("邮箱不合法"))?;
    if let Some(ref avatar) = comment.avatar {
        RE_URL
            .is_match(&avatar)
            .then(|| ())
            .ok_or(ErrorBadRequest("头像url不合法"))?;
    }
    let mut active_comment = comment::ActiveModel::from(comment);

    active_comment.id = NotSet;
    active_comment.blog_id = Set(blog_id);
    let res = active_comment.insert(&conn as &DatabaseConnection).await?;
    Ok(web::Json(json!({"msg":"回复成功","data":res})))
}

#[get("/{blog_id}/{page_size}/{index}")]
async fn comments(
    path: web::Path<(i32, usize, usize)>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    let path = path.into_inner();
    let (res, pages, items) = dbcomment::get_comments(path, &conn).await?;
    Ok(web::Json(json!({
        "msg":"获取成功",
        "data":{
            "list": res,
            "pages":pages,
            "items":items,
        }
    })))
}

#[delete("/{comment_id}")]
async fn delete_comment(
    id: Identity,
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> ServerResult<impl Responder> {
    try_verify(&id, 3)?;
    let comment_id = path.into_inner();
    let _res = comment::Entity::delete_by_id(comment_id)
        .exec(&conn as &DatabaseConnection)
        .await?;
    Ok(web::Json(json!({"msg":"删除成功"})))
}
