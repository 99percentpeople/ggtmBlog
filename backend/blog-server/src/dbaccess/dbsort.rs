use super::*;

#[derive(FromQueryResult, Serialize)]
pub struct SortAndBlogCount {
    id: i32,
    name: String,
    blog_count: i64,
}
pub async fn get_list_with_query(
    query: &SortQuery,
    path: (u64, u64),
    conn: &DatabaseConnection,
) -> ServerResult<(Vec<SortAndBlogCount>, u64, u64)> {
    let (page_size, index) = path;

    let pages = sort::Entity::find()
        .column_as(blog::Column::Id.count(), "blog_count")
        .join_rev(JoinType::LeftJoin, blog::Relation::Sort.def())
        .group_by(sort::Column::Id)
        .filter(
            Condition::any()
                .add_option(match query.name.to_owned() {
                    Some(name) => Some(sort::Column::Name.like(&format!("%{name}%"))),
                    None => None,
                })
                .add_option(match query.published {
                    Some(val @ true) => Some(blog::Column::Published.eq(val)),
                    _ => None,
                }),
        )
        .order_by_asc(sort::Column::Id)
        .into_model::<SortAndBlogCount>()
        .paginate(conn, page_size);

    let (data, pages, items) = try_join!(
        pages.fetch_page(index),
        pages.num_pages(),
        pages.num_items(),
    )?;

    Ok((data, pages, items))
}

pub async fn get_one_by_id(id: i32, conn: &DatabaseConnection) -> ServerResult<sort::Model> {
    let sort = sort::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    Ok(sort)
}

pub async fn insert_sort(
    sort: sort::Model,
    conn: &DatabaseConnection,
) -> ServerResult<sort::Model> {
    let res = sort::ActiveModel {
        name: Set(sort.name),
        ..Default::default()
    }
    .insert(conn)
    .await?;
    Ok(res)
}

pub async fn update_sort(
    id: i32,
    sort: sort::Model,
    conn: &DatabaseConnection,
) -> ServerResult<sort::Model> {
    let old = sort::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    let sort = sort::ActiveModel {
        id: Set(old.id),
        name: Set(sort.name),
    }
    .update(conn)
    .await?;

    Ok(sort)
}

pub async fn delete_sort(id: i32, conn: &DatabaseConnection) -> ServerResult<()> {
    let old = sort::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    sort::ActiveModel::from(old).delete(conn).await?;

    Ok(())
}
