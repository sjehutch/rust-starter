use crate::{db::Db, models::Item};
use axum::{Json, extract::State};

// GET /items
pub async fn list_items(State(db): State<Db>) -> Json<Vec<Item>> {
    let items: Vec<Item> = db.read().unwrap().values().cloned().collect();
    Json(items)
}

// POST /items
pub async fn create_item(State(db): State<Db>, Json(item): Json<Item>) {
    db.write().unwrap().insert(item.id.clone(), item);
}
