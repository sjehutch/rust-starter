use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::domain::{Item, ItemError};
use super::AppState;

#[utoipa::path(
    get,
    path = "/items",
    responses(
        (status = 200, description = "List all items", body = [Item])
    ),
    tag = "items"
)]
pub async fn list_items(State(state): State<AppState>) -> Json<Vec<Item>> {
    let items: Vec<Item> = state.items.list();
    Json(items)
}

#[utoipa::path(
    post,
    path = "/items",
    request_body = Item,
    responses(
        (status = 201, description = "Item created"),
        (status = 409, description = "Item already exists")
    ),
    tag = "items"
)]
pub async fn create_item(State(state): State<AppState>, Json(item): Json<Item>) -> Result<StatusCode, (StatusCode, String)> {
    match state.items.create(item) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(ItemError::Conflict) => Err((StatusCode::CONFLICT, "Item already exists".to_string())),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/items", get(list_items).post(create_item))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::items::ItemService,
        infra::{repo::memory::InMemoryItemRepo, storage::FileStorage},
    };
    use axum::{
        body::{self, Body},
        http::{Request, StatusCode},
    };
    use std::sync::Arc;
    use tempfile::tempdir;
    use tower::ServiceExt;

    #[tokio::test]
    async fn create_and_list_items_round_trip() {
        let repo = Arc::new(InMemoryItemRepo::new_shared());
        let state = AppState {
            items: ItemService::new(repo),
            storage: FileStorage::new(tempdir().unwrap().path()),
        };
        let app = super::routes().with_state(state);

        let new_item = r#"{"id":"1","name":"Widget","age":null}"#;
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/items")
                    .header("content-type", "application/json")
                    .body(Body::from(new_item))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response = app
            .oneshot(Request::builder().uri("/items").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = body::to_bytes(response.into_body(), 1024).await.unwrap();
        let items: Vec<Item> = serde_json::from_slice(&body).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Widget");
    }
}
