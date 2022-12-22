use sea_orm::{
    sea_query::{Expr, Func},
};

use super::*;

use crate::{
    error::Error,
    model::{BlogDetailItem, BlogListItem, BlogQuery, BlogSearch, UpdateBlog, UserBriefInfo},
};

pub async fn blog_search(
    query: &BlogQuery,
    conn: &DatabaseConnection,
    published: bool,
) -> ServerResult<Vec<BlogSearch>> {
    let res = blog::Entity::find()
        .select_only()
        .column(blog::Column::Id)
        .column(blog::Column::Title)
        .filter(
            Condition::all()
                .add_option(match &query.title_or_content {
                    Some(v) => Some(
                        Expr::expr(Func::upper(Expr::col(blog::Column::Title)))
                            .like(format!("%{v}%").to_uppercase())
                            .or(Expr::expr(Func::upper(Expr::col(blog::Column::Content)))
                                .like(format!("%{v}%").to_uppercase()))
                            .or(Expr::expr(Func::upper(Expr::col(blog::Column::Summary)))
                                .like(format!("%{v}%").to_uppercase())),
                    ),
                    None => None,
                })
                .add_option(match published {
                    true => Some(blog::Column::Published.eq(true)),
                    false => None,
                }),
        )
        .limit(10)
        .into_model::<BlogSearch>()
        .all(conn)
        .await?;

    Ok(res)
}

pub async fn get_model_list_with_query(
    query: &BlogQuery,
    path: (u64, u64),
    conn: &DatabaseConnection,
    published: bool,
) -> ServerResult<(Vec<blog::Model>, u64, u64)> {
    let (page_size, index) = path;

    let pages = blog::Entity::find()
        .filter(
            Condition::all()
                .add_option(match &query.title {
                    Some(val) => Some(blog::Column::Title.like(&format!("%{val}%"))),
                    None => None,
                })
                .add_option(match &query.title_or_content {
                    Some(val) => Some(
                        Condition::any()
                            .add(blog::Column::Title.like(&format!("%{val}%")))
                            .add(blog::Column::Content.like(&format!("%{val}%"))),
                    ),
                    None => None,
                })
                .add_option(match query.sort_id {
                    Some(val) => Some(blog::Column::SortId.eq(val)),
                    None => None,
                })
                .add_option(match query.recommend {
                    Some(val @ true) => Some(blog::Column::Recommend.eq(val)),
                    _ => None,
                })
                .add_option(match &query.tag_ids {
                    Some(tag_ids) if !tag_ids.is_empty() => {
                        let ids = tag_ids
                            .split(",")
                            .map(|v| v.parse())
                            .collect::<Result<Vec<i32>, _>>()?;

                        Some(sea_query::Expr::col(blogs_tags::Column::TagId).is_in(ids))
                    }
                    _ => None,
                })
                .add_option(match published {
                    true => Some(blog::Column::Published.eq(true)),
                    false => None,
                }),
        )
        .column_as(blogs_tags::Column::TagId.max(), "tag_id")
        .join_rev(JoinType::LeftJoin, blogs_tags::Relation::Blog.def())
        .group_by(blog::Column::Id)
        .order_by(
            match &query.sort_by {
                Some(val) if val == "update_time" => blog::Column::UpdateTime,
                Some(val) if val == "create_time" => blog::Column::CreateTime,
                _ => blog::Column::Id,
            },
            match &query.order {
                Some(val) if val == "asc" => Order::Asc,
                Some(val) if val == "desc" => Order::Desc,
                _ => Order::Asc,
            },
        )
        .paginate(conn, page_size);

    let (data, pages, items) = try_join!(
        pages.fetch_page(index),
        pages.num_pages(),
        pages.num_items(),
    )?;

    let data = data
        .into_iter()
        .map(|mut item| {
            if let Some(false) | None = query.summary {
                item.summary = None;
            }
            if let Some(false) | None = query.cover {
                item.cover = None;
            }
            item
        })
        .collect();

    Ok((data, pages, items))
}

pub async fn get_item_list_with_query(
    query: &BlogQuery,
    path: (u64, u64),
    conn: &DatabaseConnection,
    published: bool,
) -> ServerResult<(Vec<BlogListItem>, u64, u64)> {
    let (data, pages, items) = get_model_list_with_query(query, path, conn, published).await?;
    let handles = data.into_iter().map(|blog| {
        let conn = conn.to_owned();
        rt::spawn(async move {
            let (sort, user, tags) = try_join!(
                blog.find_related(sort::Entity).one(&conn),
                blog.find_related(user::Entity)
                    .into_model::<UserBriefInfo>()
                    .one(&conn),
                blog.find_related(tag::Entity).all(&conn)
            )?;
            Ok::<_, Error>(BlogListItem {
                sort: sort.map_or(None, |sort| Some(sort.into())),
                user: user.map_or(None, |user| Some(user.into())),
                tags,
                ..blog.into()
            })
        })
    });

    let collection = future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<ServerResult<Vec<_>>>()?;

    Ok((collection, pages, items))
}

pub async fn insert_blog(
    blog: blog::Model,
    conn: &DatabaseConnection,
) -> ServerResult<blog::Model> {
    let tag_ids = blog.tag_ids.to_owned();

    let active_blog = blog::ActiveModel::from(blog);

    let res = active_blog.insert(conn).await?;
    let id = res.id;
    let handles = tag_ids.into_iter().map(|tag_id| {
        let conn = conn.to_owned();
        rt::spawn(async move {
            Ok::<_, Error>(
                blogs_tags::ActiveModel::from(blogs_tags::Model::from((id, tag_id)))
                    .insert(&conn)
                    .await?,
            )
        })
    });

    future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<ServerResult<Vec<_>>>()?;

    Ok(res)
}

pub async fn update_blog(
    id: i32,
    blog: UpdateBlog,
    conn: &DatabaseConnection,
) -> ServerResult<blog::Model> {
    let tag_ids = blog.tag_ids.clone();
    let old_blog = blog::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    let new_blog = blog.into_blog(&old_blog).update(conn).await?;

    if let Some(tag_ids) = tag_ids {
        let coll = new_blog.find_related(tag::Entity).all(conn).await?;

        let handles1 = coll.into_iter().map(|tag::Model { id: tag_id, .. }| {
            let conn = conn.to_owned();
            rt::spawn(async move {
                Ok::<_, Error>(blogs_tags::Model::from((id, tag_id)).delete(&conn).await?)
            })
        });
        let handles2 = tag_ids.into_iter().map(|tag_id| {
            let conn = conn.to_owned();
            rt::spawn(async move {
                Ok::<_, Error>(
                    blogs_tags::ActiveModel::from(blogs_tags::Model::from((id, tag_id)))
                        .insert(&conn)
                        .await?,
                )
            })
        });
        /* 先删除所有标签 */
        future::try_join_all(handles1)
            .await?
            .into_iter()
            .collect::<ServerResult<Vec<_>>>()?;
        /* 再添加相应标签 */
        future::try_join_all(handles2)
            .await?
            .into_iter()
            .collect::<ServerResult<Vec<_>>>()?;
    }

    Ok(new_blog)
}

pub async fn delete_blog(id: i32, conn: &DatabaseConnection) -> ServerResult<()> {
    blog::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?
        .delete(conn)
        .await?;

    Ok(())
}

pub async fn get_one_by_id(
    id: i32,
    query: BlogQuery,
    conn: &DatabaseConnection,
) -> ServerResult<BlogDetailItem> {
    let blog = blog::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::RecordNotFound("没有找到".to_owned()))?;

    if let Some(true) = query.view {
        let mut active = blog::ActiveModel::from(blog.to_owned());
        active.views = Set(blog.views + 1);
        active.update(conn).await?;
    }

    let (sort, tags, user) = try_join!(
        blog.find_related(sort::Entity).one(conn),
        blog.find_related(tag::Entity).all(conn),
        blog.find_related(user::Entity).one(conn)
    )?;
    Ok(BlogDetailItem {
        tags,
        sort,
        user: user.map_or(None, |user| Some(user.into())),
        ..blog.into()
    })
}
