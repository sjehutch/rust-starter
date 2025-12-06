mod application;
mod domain;
mod infra;

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::sync::Arc;

use crate::{
    application::items::ItemService,
    domain::Item,
    infra::{
        http::{self, AppState},
        repo::memory::InMemoryItemRepo,
        storage::FileStorage,
    },
    infra::http::uploads::{UploadBody, UploadResponse},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        http::items::list_items,
        http::items::create_item,
        http::uploads::upload_image,
    ),
    components(schemas(Item, UploadResponse, UploadBody)),
    tags(
        (name = "items", description = "Item operations"),
        (name = "uploads", description = "File upload operations")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let repo = InMemoryItemRepo::new_shared();
    let state = AppState {
        items: ItemService::new(Arc::new(repo)),
        storage: FileStorage::new("uploads"),
    };

    let app = http::router(state.clone())
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        );
    println!("🚀 API running at http://localhost:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
