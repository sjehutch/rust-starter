use axum::{Router, routing::get};
use crate::application::items::ItemService;
use crate::infra::storage::FileStorage;

pub mod items;
pub mod uploads;

#[derive(Clone)]
pub struct AppState {
    pub items: ItemService,
    pub storage: FileStorage,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/", items::routes())
        .nest("/", uploads::routes())
        .with_state(state)
        .route("/health", get(|| async { "ok" }))
}
