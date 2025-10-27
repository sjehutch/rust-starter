use crate::models::Item;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

// Shared state type alias
pub type Db = Arc<RwLock<HashMap<String, Item>>>;

pub trait DbExt {
    fn new() -> Db;
}

impl DbExt for Db {
    fn new() -> Db {
        Arc::new(RwLock::new(HashMap::new()))
    }
}
