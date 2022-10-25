use actix_web::{body::BoxBody, HttpResponse, Responder};
use chrono::NaiveDateTime;
use entity::{blog, comment, tag, user};
use sea_orm::{FromQueryResult, Set, Unchanged};
use serde::{Deserialize, Serialize};

use entity::sort;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub access_level: Option<i32>,
}
#[derive(Serialize, Clone, FromQueryResult)]
pub struct UserBriefInfo {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}
impl From<user::Model> for UserInfo {
    fn from(user_model: user::Model) -> Self {
        UserInfo {
            id: user_model.id,
            username: user_model.username,
            nickname: user_model.nickname,
            email: user_model.email,
            avatar: user_model.avatar,
            access_level: user_model.access_level,
            create_time: user_model.create_time,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateBlog {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub cover: Option<String>,
    pub flag: Option<blog::Flag>,
    pub views: Option<i32>,
    pub appreciation: Option<bool>,
    pub share_statement: Option<bool>,
    pub enable_comment: Option<bool>,
    pub published: Option<bool>,
    pub recommend: Option<bool>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub sort_id: Option<i32>,
    pub tag_ids: Option<Vec<i32>>,
}

impl UpdateBlog {
    pub fn into_blog(self, old: &blog::Model) -> blog::ActiveModel {
        blog::ActiveModel {
            id: Unchanged(old.id),
            user_id: Unchanged(old.user_id),
            title: self
                .title
                .map_or(Unchanged(old.title.to_owned()), |val| Set(val)),
            summary: self
                .summary
                .map_or(Unchanged(old.summary.to_owned()), |val| Set(Some(val))),
            content: self
                .content
                .map_or(Unchanged(old.content.to_owned()), |val| Set(val)),
            cover: self
                .cover
                .map_or(Unchanged(old.cover.to_owned()), |val| Set(Some(val))),
            flag: self
                .flag
                .map_or(Unchanged(old.flag.to_owned()), |val| Set(val)),
            views: self.views.map_or(Unchanged(old.views), |val| Set(val)),
            appreciation: self
                .appreciation
                .map_or(Unchanged(old.appreciation), |val| Set(val)),
            share_statement: self
                .share_statement
                .map_or(Unchanged(old.share_statement), |val| Set(val)),
            enable_comment: self
                .enable_comment
                .map_or(Unchanged(old.enable_comment), |val| Set(val)),
            published: self
                .published
                .map_or(Unchanged(old.published), |val| Set(val)),
            recommend: self
                .recommend
                .map_or(Unchanged(old.recommend), |val| Set(val)),
            create_time: self
                .create_time
                .map_or(Unchanged(old.create_time), |val| Set(Some(val))),
            update_time: self
                .update_time
                .map_or(Unchanged(old.update_time), |val| Set(Some(val))),
            sort_id: self.sort_id.map_or(Unchanged(old.sort_id), |val| Set(val)),
        }
    }
}
#[derive(Debug, Serialize, FromQueryResult)]
pub struct BlogSearch {
    id: i32,
    title: String,
}
#[derive(Deserialize, Debug)]
pub struct BlogQuery {
    pub view: Option<bool>,
    pub title: Option<String>,
    pub title_or_content: Option<String>,
    pub sort_id: Option<i32>,
    pub order: Option<String>,
    pub sort_by: Option<String>,
    pub recommend: Option<bool>,
    pub summary: Option<bool>,
    pub cover: Option<bool>,
    pub tag_ids: Option<String>,
}

#[derive(Deserialize)]
pub struct SortQuery {
    pub name: Option<String>,
    pub published: Option<bool>,
}

#[derive(Deserialize)]
pub struct TagQuery {
    pub name: Option<String>,
    pub published: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct JsonResponse<T: Serialize = serde_json::Value> {
    pub msg: String,
    pub data: T,
}

impl<T: Serialize> Responder for JsonResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Serialize, Clone)]
pub struct BlogListItem {
    pub id: i32,
    pub title: String,
    pub summary: Option<String>,
    pub recommend: bool,
    pub published: bool,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub cover: Option<String>,
    pub views: i32,
    pub flag: blog::Flag,
    pub sort: Option<sort::Model>,
    pub user: Option<UserBriefInfo>,
    pub tags: Vec<tag::Model>,
}

impl From<blog::Model> for BlogListItem {
    fn from(blog: blog::Model) -> Self {
        Self {
            id: blog.id,
            title: blog.title,
            summary: blog.summary,
            recommend: blog.recommend,
            create_time: blog.create_time,
            update_time: blog.update_time,
            published: blog.published,
            views: blog.views,
            flag: blog.flag,
            cover: blog.cover,
            sort: None,
            user: None,
            tags: vec![],
        }
    }
}

pub struct SimpleBlogListItem {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize)]
pub struct BlogDetailItem {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub flag: blog::Flag,
    pub views: i32,
    pub appreciation: bool,
    pub share_statement: bool,
    pub enable_comment: bool,
    pub published: bool,
    pub recommend: bool,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub tags: Vec<tag::Model>,
    pub sort: Option<sort::Model>,
    pub user: Option<UserInfo>,
}

impl From<blog::Model> for BlogDetailItem {
    fn from(blog: blog::Model) -> Self {
        Self {
            id: blog.id,
            title: blog.title,
            content: blog.content,
            cover: blog.cover,
            flag: blog.flag,
            views: blog.views,
            appreciation: blog.appreciation,
            share_statement: blog.share_statement,
            enable_comment: blog.enable_comment,
            published: blog.published,
            recommend: blog.recommend,
            create_time: blog.create_time,
            update_time: blog.update_time,
            summary: blog.summary,
            tags: vec![],
            sort: None,
            user: None,
        }
    }
}
#[derive(Deserialize)]
pub struct CommentQuery {
    pub order: Option<String>,
    pub sort_by: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RecursiveComment {
    pub comment: comment::Model,
    pub sub_comments: Vec<RecursiveComment>,
}
#[derive(Deserialize, Serialize)]
pub struct Comments {
    pub comment: comment::Model,
    pub sub_comments: Vec<comment::Model>,
}

impl From<RecursiveComment> for Vec<comment::Model> {
    fn from(rc: RecursiveComment) -> Self {
        [
            vec![rc.comment],
            rc.sub_comments
                .into_iter()
                .flat_map(|c| -> Vec<comment::Model> { c.into() })
                .collect::<Vec<comment::Model>>(),
        ]
        .concat()
    }
}
