use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    model::AppError,
    services::post::{
        CreatePostRequest, ListPostsResponse, PostService, SinglePostResponse, UpdatePostRequest,
    },
    state::ApplicationState,
};

#[axum::debug_handler]
pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListPostsResponse>, AppError> {
    let posts = state.post_service.get_all_posts().await?;

    let response = ListPostsResponse { data: posts };

    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn create(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.create_post(payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn update(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.update_post(&id, payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<()>, AppError> {
    state.post_service.delete_post(id).await?;

    Ok(Json(()))
}

pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(slug): Path<String>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.get_post_by_slug(&slug).await;

    match post {
        Ok(post) => {
            let response = SinglePostResponse { data: post };

            Ok(Json(response))
        }
        Err(_) => Err(AppError::NotFound {
            item_type: crate::model::Models::PostModel,
            id: slug,
        }),
    }
}
