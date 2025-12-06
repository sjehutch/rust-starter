use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

/// Core item entity.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub age: Option<u32>,
}

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("item already exists")]
    Conflict,
}

/// Repository abstraction for items.
pub trait ItemRepo: Send + Sync {
    fn list(&self) -> Vec<Item>;
    fn insert(&self, item: Item) -> Result<(), ItemError>;
}
