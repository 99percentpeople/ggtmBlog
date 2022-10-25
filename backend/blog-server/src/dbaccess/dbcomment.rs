use super::*;
use crate::{
    error::ServerResult,
    model::{Comments, RecursiveComment},
};
use actix_web::{error::ErrorNotFound, rt};
use async_recursion::async_recursion;


pub async fn get_comments(
    path: (i32, usize, usize),
    conn: &DatabaseConnection,
) -> ServerResult<(Vec<Comments>, usize, usize)> {
    let (blog_id, page_size, index) = path;
    let blog = blog::Entity::find_by_id(blog_id)
        .one(conn)
        .await?
        .ok_or(ErrorNotFound(format!("{blog_id} 不存在")))?;
    let pages = blog
        .find_related(comment::Entity)
        .group_by(comment::Column::Id)
        .having(comment::Column::IdRef.is_null())
        .order_by(comment::Column::CreateTime, Order::Desc)
        .paginate(conn, page_size);
    let (data, pages, items) = try_join!(
        pages.fetch_page(index),
        pages.num_pages(),
        pages.num_items(),
    )?;

    let handles = data.into_iter().map(|comment| {
        let conn = conn.to_owned();
        rt::spawn(async move {
            Ok(RecursiveComment {
                sub_comments: self::get_sub_comments(&comment, &conn).await?,
                comment,
            })
        })
    });

    let collection = future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<ServerResult<Vec<_>>>()?;

    let res: Vec<Comments> = collection
        .into_iter()
        .map(|e| Comments {
            comment: e.comment.clone(),
            sub_comments: e
                .sub_comments
                .into_iter()
                .flat_map(|c| -> Vec<comment::Model> { c.into() })
                .collect(),
        })
        .collect();
    Ok((res, pages, items))
}

#[async_recursion]
pub async fn get_sub_comments(
    comment: &comment::Model,
    conn: &DatabaseConnection,
) -> ServerResult<Vec<RecursiveComment>> {
    let sub_comment = comment
        .find_linked(comment::SubCommentLink)
        .all(conn)
        .await?;

    let handles = sub_comment.into_iter().map(|comment| {
        let conn = conn.to_owned();
        rt::spawn(async move {
            Ok(RecursiveComment {
                sub_comments: get_sub_comments(&comment, &conn).await?,
                comment,
            })
        })
    });

    let res = future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<ServerResult<Vec<_>>>()?;

    Ok(res)
}
