use super::*;

use serde::Serialize;

#[derive(FromQueryResult, Serialize)]
pub struct TagAndBlogCount {
    id: i32,
    name: String,
    blog_count: i64,
}
pub async fn get_list_with_query(
    query: &TagQuery,
    path: (u64, u64),
    conn: &DatabaseConnection,
) -> ServerResult<(Vec<TagAndBlogCount>, u64, u64)> {
    let (page_size, index) = path;

    let pages = tag::Entity::find()
        .column_as(blogs_tags::Column::TagId.count(), "blog_count")
        .join_rev(JoinType::LeftJoin, blogs_tags::Relation::Tag.def())
        .join(JoinType::LeftJoin, blogs_tags::Relation::Blog.def())
        .group_by(tag::Column::Id)
        .filter(
            Condition::any()
                .add_option(match query.name.to_owned() {
                    Some(name) => Some(tag::Column::Name.like(&format!("%{name}%"))),
                    None => None,
                })
                .add_option(match query.published {
                    Some(val @ true) => Some(blog::Column::Published.eq(val)),
                    _ => None,
                }),
        )
        .order_by_asc(tag::Column::Id)
        .into_model::<TagAndBlogCount>()
        .paginate(conn, page_size);

    let (data, pages, items) = try_join!(
        pages.fetch_page(index),
        pages.num_pages(),
        pages.num_items(),
    )?;

    Ok((data, pages, items))
}

pub async fn get_one_by_id(id: i32, conn: &DatabaseConnection) -> ServerResult<tag::Model> {
    let tag = tag::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    Ok(tag)
}

pub async fn insert_tag(tag: tag::Model, conn: &DatabaseConnection) -> ServerResult<tag::Model> {
    let res = tag::ActiveModel {
        name: Set(tag.name),
        ..Default::default()
    }
    .insert(conn)
    .await?;
    Ok(res)
}

pub async fn update_tag(
    id: i32,
    tag: tag::Model,
    conn: &DatabaseConnection,
) -> ServerResult<tag::Model> {
    let old = tag::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    let tag = tag::ActiveModel {
        id: Set(old.id),
        name: Set(tag.name),
    }
    .update(conn)
    .await?;

    Ok(tag)
}

pub async fn delete_tag(id: i32, conn: &DatabaseConnection) -> ServerResult<()> {
    let old = tag::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    tag::ActiveModel::from(old).delete(conn).await?;

    Ok(())
}
