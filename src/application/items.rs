use std::sync::Arc;

use crate::domain::{Item, ItemError, ItemRepo};

#[derive(Clone)]
pub struct ItemService {
    repo: Arc<dyn ItemRepo>,
}

impl ItemService {
    pub fn new(repo: Arc<dyn ItemRepo>) -> Self {
        Self { repo }
    }

    pub fn list(&self) -> Vec<Item> {
        self.repo.list()
    }

    pub fn create(&self, item: Item) -> Result<(), ItemError> {
        self.repo.insert(item)
    }
}
