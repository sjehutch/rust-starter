use std::path::Path;

use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::post,
    Json, Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::AppState;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UploadResponse {
    pub id: String,
    pub path: String,
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub struct UploadBody {
    /// File upload field named "file"
    #[schema(value_type = String, format = Binary)]
    pub file: Vec<u8>,
}

/// POST /upload
#[utoipa::path(
    post,
    path = "/upload",
    request_body(
        content = UploadBody,
        content_type = "multipart/form-data",
        description = "Upload an image file in form field `file`"
    ),
    responses(
        (status = 201, description = "File uploaded", body = UploadResponse),
        (status = 400, description = "Invalid upload"),
        (status = 500, description = "Failed to store file"),
    ),
    tag = "uploads"
)]
pub async fn upload_image(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadResponse>), (StatusCode, String)> {
    let mut file_bytes = None;
    let mut file_name = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid multipart data: {e}")))? 
    {
        if field.name() == Some("file") {
            file_name = field.file_name().map(|s| s.to_string());
            let data = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read file bytes: {e}")))?;
            file_bytes = Some(data);
            break;
        }
    }

    let data = file_bytes.ok_or((StatusCode::BAD_REQUEST, "Missing form field `file`".to_string()))?;

    let (id, full_path) = state
        .storage
        .save(file_name.as_deref(), &data)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    tracing::info!(file = %full_path.display(), "Image uploaded to directory");

    let response = UploadResponse {
        id,
        path: full_path.display().to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/upload", post(upload_image))
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
    use std::{env, sync::Arc};
    use tempfile::tempdir;
    use tower::ServiceExt;

    #[tokio::test]
    async fn upload_saves_file() {
        let temp = tempdir().unwrap();
        env::set_current_dir(&temp).unwrap();

        let repo = Arc::new(InMemoryItemRepo::new_shared());
        let state = AppState {
            items: ItemService::new(repo),
            storage: FileStorage::new("uploads"),
        };

        let app = super::routes().with_state(state);

        let boundary = "XBOUNDARY";
        let form_body = format!(
            "--{b}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.png\"\r\nContent-Type: image/png\r\n\r\n{data}\r\n--{b}--\r\n",
            b = boundary,
            data = "PNGDATA"
        );

        let request = Request::builder()
            .method("POST")
            .uri("/upload")
            .header(
                "content-type",
                format!("multipart/form-data; boundary={}", boundary),
            )
            .body(Body::from(form_body))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = body::to_bytes(response.into_body(), 1024).await.unwrap();
        let parsed: UploadResponse = serde_json::from_slice(&body).unwrap();

        let saved_path = Path::new(&parsed.path);
        assert!(saved_path.exists(), "file should have been written");
        assert_eq!(saved_path.parent().unwrap().file_name().unwrap(), "uploads");
    }
}
