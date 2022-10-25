use super::*;
use crate::model::LoginForm;
use crate::{error::ServerResult, model::UserInfo};
use actix_identity::Identity;
use actix_web::get;
use actix_web::{
    error::{ErrorBadRequest, ErrorUnauthorized},
    post, services, web, Responder,
};
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
pub fn auth_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(services![login, logout, post_verify, get_user_info]);
}

#[post("/verify/{sort}")]
async fn post_verify(id: Identity, sort: web::Path<i32>) -> ServerResult<impl Responder> {
    try_verify(&id, sort.into_inner())?;

    Ok(web::Json(json!({"msg":"认证成功"})))
}

#[post("/login")]
async fn login(
    id: Identity,
    conn: web::Data<DatabaseConnection>,
    user: web::Json<LoginForm>,
) -> ServerResult<impl Responder> {
    if let Some(_) = id.identity() {
        Ok(web::Json(json!({"msg":"已经登录了"})))
    } else {
        let user = user.into_inner();
        let user_info: UserInfo = user::Entity::find()
            .filter(user::Column::Username.eq(user.username))
            .filter(user::Column::Password.eq(user.password))
            .one(&conn as &DatabaseConnection)
            .await?
            .ok_or(ErrorBadRequest("登录失败"))?
            .into();

        id.remember(serde_json::to_string(&user_info).unwrap());
        Ok(web::Json(json!({"msg":"登录成功","data":user_info})))
    }
}

#[post("/logout")]
async fn logout(id: Identity) -> ServerResult<impl Responder> {
    try_verify(&id, 0)?;
    id.forget();
    Ok(web::Json(json!({"msg":"注销成功"})))
}

#[get("/user")]
async fn get_user_info(id: Identity) -> ServerResult<impl Responder> {
    let user_info: UserInfo =
        serde_json::from_str(&id.identity().ok_or(ErrorUnauthorized("未登录"))?)
            .map_err(|err| ErrorBadRequest(format!("cookie解析失败: {}", err)))?;

    Ok(web::Json(json!({"msg":"获取成功","data":user_info})))
}
