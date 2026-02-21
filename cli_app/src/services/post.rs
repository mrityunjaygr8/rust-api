use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::model::{Post, PostStatus};

#[allow(async_fn_in_trait)]
pub trait PostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>>;
    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post>;
    async fn get_post_by_slug(&self, name: &str) -> anyhow::Result<Post>;
    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post>;
    async fn update_post(&self, id: &i64, req: UpdatePostRequest) -> anyhow::Result<Post>;
    async fn delete_post(&self, id: i64) -> anyhow::Result<()>;
}

#[derive(Clone, Deserialize)]
pub struct CreatePostRequest {
    pub author_id: i64,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
}

#[derive(Clone, Deserialize)]
pub struct UpdatePostRequest {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
}

#[derive(Serialize)]
pub struct SinglePostResponse {
    pub data: Post,
}

impl IntoResponse for SinglePostResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

#[derive(Serialize)]
pub struct ListPostsResponse {
    pub data: Vec<Post>,
}
impl IntoResponse for ListPostsResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
