use std::path::{Path, PathBuf};

use tokio::fs;
use uuid::Uuid;

#[derive(Clone)]
pub struct FileStorage {
    base: PathBuf,
}

impl FileStorage {
    pub fn new<P: Into<PathBuf>>(base: P) -> Self {
        Self { base: base.into() }
    }

    #[allow(dead_code)]
    pub fn base_dir(&self) -> &Path {
        &self.base
    }

    pub async fn save(&self, original_name: Option<&str>, data: &[u8]) -> Result<(String, PathBuf), String> {
        let ext = original_name
            .and_then(|name| Path::new(name).extension())
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");

        let id = Uuid::new_v4().to_string();
        let filename = format!("{id}.{ext}");
        let dir = &self.base;

        fs::create_dir_all(dir)
            .await
            .map_err(|e| format!("Failed to create upload dir: {e}"))?;

        let full_path = dir.join(&filename);
        fs::write(&full_path, data)
            .await
            .map_err(|e| format!("Failed to write file: {e}"))?;

        Ok((id, full_path))
    }
}
