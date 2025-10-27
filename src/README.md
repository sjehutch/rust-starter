# 🦀 Inventory API (Rust + Axum)

A simple high-performance REST API built in **Rust** using:

- **Axum** for routing
- **Tokio** for async runtime
- **Serde** for JSON serialization
- In-memory data store (`Arc<RwLock<HashMap<..>>>`)

This project is intentionally structured to help you **learn Rust backend development** the right way: clean modules, clear separation of concerns, and ready for real database integration (SQLite/Postgres).

---

## ✅ Features

| Endpoint | Method | Description       |
| -------- | ------ | ----------------- |
| `/items` | `GET`  | List all items    |
| `/items` | `POST` | Create a new item |

**Example request to create an item:**

```json
{
  "id": "001",
  "name": "Hammer"
}
```

---

## 📦 Project Structure

```
inventory-api/
  Cargo.toml
  src/
    main.rs       # Server startup + route configuration
    routes.rs     # HTTP handlers
    models.rs     # Request/Response DTOs
    db.rs         # Shared in-memory store (HashMap wrapped in Arc/RwLock)
```

---

## 🚀 Run the Server

```sh
cargo run
```

The API will start on:

```
http://localhost:3000
```

---

## 🧪 Test the API

### Create an item

```sh
curl -X POST http://localhost:3000/items \
  -H "Content-Type: application/json" \
  -d '{"id":"001","name":"Hammer"}'
```

### List all items

```sh
curl http://localhost:3000/items
```

Expected result:

```json
[{ "id": "001", "name": "Hammer" }]
```

---

## 🧠 Technology Overview

| Library          | Purpose                                             |
| ---------------- | --------------------------------------------------- |
| `axum`           | Routing & HTTP handling (like ASP.NET Minimal APIs) |
| `tokio`          | Async runtime used for tasks, networking, IO        |
| `serde`          | JSON serialization/deserialization                  |
| `serde_json`     | JSON encoding                                       |
| `Arc<RwLock<T>>` | Thread-safe shared state across requests            |

This backend uses **zero garbage collector**, and Rust's **ownership model** prevents data races and memory errors at compile time.

---

## 🧱 Learning Goals

This project teaches:

- Shared state using `Arc<RwLock<T>>`
- `async` in Rust
- JSON route handling
- Clean module structure
- "No GC" memory model thinking

---

## 🔧 Development TO-DO List

| Status                                                 | Task |
| ------------------------------------------------------ | ---- |
| [ ] ✅ Add `GET /items/{id}` route                     |
| [ ] ✅ Add `DELETE /items/{id}` endpoint               |
| [ ] ✅ Add logging middleware (tracing + tower)        |
| [ ] ✅ Convert store to SQLite using `sqlx`            |
| [ ] (Optional) Add pagination support                  |
| [ ] (Optional) Add API authentication                  |
| [ ] (Optional) Add metrics endpoint (Prometheus style) |

---

## 🧠 Your Next Step — Choose Progress Path

Reply with one option:

**A**) Add `/items/{id}`  
**B**) Add DELETE  
**C**) Convert to SQLite  
**D**) Do all of the above (step-by-step)

---

## 🧒 ELI5 Summary

This little server is a **brain** that remembers items you tell it about.  
You talk to it using web requests.  
Rust makes sure the brain **never crashes**, never does something unsafe, and stays **fast**.

```
cargo run
curl /items
```

That’s the whole magic. 🧠⚡
