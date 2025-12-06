use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::domain::{Item, ItemError, ItemRepo};

pub type Db = Arc<RwLock<HashMap<String, Item>>>;

#[derive(Clone, Default)]
pub struct InMemoryItemRepo {
    db: Db,
}

impl InMemoryItemRepo {
    #[allow(dead_code)]
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn new_shared() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl ItemRepo for InMemoryItemRepo {
    fn list(&self) -> Vec<Item> {
        self.db.read().unwrap().values().cloned().collect()
    }

    fn insert(&self, item: Item) -> Result<(), ItemError> {
        let mut write_guard = self.db.write().unwrap();
        if write_guard.contains_key(&item.id) {
            return Err(ItemError::Conflict);
        }
        write_guard.insert(item.id.clone(), item);
        Ok(())
    }
}
