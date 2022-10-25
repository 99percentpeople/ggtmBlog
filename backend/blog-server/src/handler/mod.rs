use crate::{error::ServerResult, model::UserInfo};
use actix_identity::Identity;
use sea_orm::DatabaseConnection;
use actix_web::{
    delete,
    error::{
        ErrorBadRequest, ErrorForbidden, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized,
    },
    get,
    http::header,
    post, put, services, web, Responder,
};
pub mod admin_blog;
pub mod auth;
pub mod blog;
pub mod comment;
pub mod file;
pub mod sort;
pub mod tag;
pub mod user;

pub fn try_verify(id: &Identity, min_access_level: i32) -> ServerResult<UserInfo> {
    let token = &id.identity().ok_or(ErrorUnauthorized("需要认证"))?;
    log::debug!("{token}");
    let user_info: UserInfo = serde_json::from_str(token)
        .map_err(|err| ErrorBadRequest(format!("cookie解析失败: {err}")))?;

    (min_access_level
        <= user_info
            .access_level
            .ok_or(ErrorBadRequest("不合法的cookie"))?)
    .then(|| user_info)
    .ok_or(ErrorForbidden("权限不足").into())
}
