use super::*;

pub async fn get_one_by_login(
    login_form: &LoginForm,
    conn: &DatabaseConnection,
) -> ServerResult<user::Model> {
    let user: user::Model = user::Entity::find()
        .filter(user::Column::Username.eq(login_form.username.to_owned()))
        .filter(user::Column::Password.eq(login_form.password.to_owned()))
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    Ok(user)
}
