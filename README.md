To structure your Rust project similarly to how a NestJS project is structured, you can organize your code into modules and directories for better separation of concerns. NestJS typically has modules, controllers, services, and models/entities. We'll adopt a similar structure:

1. **modules**: Holds individual modules (each representing a feature).
2. **controllers**: Handles the incoming HTTP requests.
3. **services**: Contains business logic.
4. **models**: Defines data models.
5. **main.rs**: Entry point of the application.

Here's how you can organize your Rust project:

### 1. Project Structure

```
axum_sqlite_crud/
├── src/
│   ├── main.rs
│   ├── modules/
│   │   ├── items/
│   │   │   ├── mod.rs
│   │   │   ├── controller.rs
│   │   │   ├── service.rs
│   │   │   ├── model.rs
│   │   │   └── routes.rs
│   └── db.rs
│   └── lib.rs
├── migrations/
│   └── 2023-06-24-001-initial/
│       └── up.sql
├── Cargo.toml
```

### 2. Code

#### `Cargo.toml`

Add the necessary dependencies:

```toml
[dependencies]
axum = "0.6"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.6", features = ["runtime-tokio-rustls", "sqlite"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = "1.0"
```

#### `src/main.rs`

Set up the main entry point:

```rust
use axum::Router;
use std::net::SocketAddr;
use sqlx::sqlite::SqlitePool;
use axum_sqlite_crud::db::init_db;
use axum_sqlite_crud::modules::items::routes::create_item_routes;

#[tokio::main]
async fn main() {
    // Initialize database pool
    let pool = init_db().await.expect("Failed to initialize the database");

    // Create app with routes
    let app = Router::new()
        .nest("/items", create_item_routes())
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3005));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

#### `src/db.rs`

Database initialization:

```rust
use sqlx::sqlite::SqlitePool;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:database.db").await?;
    Ok(pool)
}
```

#### `src/lib.rs`

Expose modules and database:

```rust
pub mod db;
pub mod modules;
```

#### `src/modules/mod.rs`

Module declaration:

```rust
pub mod items;
```

#### `src/modules/items/mod.rs`

Items module:

```rust
pub mod controller;
pub mod service;
pub mod model;
pub mod routes;
```

#### `src/modules/items/model.rs`

Item model:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
}
```

#### `src/modules/items/service.rs`

Business logic for items:

```rust
use crate::modules::items::model::Item;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

pub async fn create_item(pool: &SqlitePool, name: String, description: String) -> Result<Item, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let item = Item { id: id.clone(), name, description };

    sqlx::query("INSERT INTO items (id, name, description) VALUES (?, ?, ?)")
        .bind(&item.id)
        .bind(&item.name)
        .bind(&item.description)
        .execute(pool)
        .await?;

    Ok(item)
}

pub async fn list_items(pool: &SqlitePool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as!(Item, "SELECT id, name, description FROM items")
        .fetch_all(pool)
        .await?;

    Ok(items)
}

pub async fn get_item(pool: &SqlitePool, id: String) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as!(Item, "SELECT id, name, description FROM items WHERE id = ?", id)
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn update_item(pool: &SqlitePool, id: String, name: String, description: String) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE items SET name = ?, description = ? WHERE id = ?")
        .bind(name)
        .bind(description)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_item(pool: &SqlitePool, id: String) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
```

#### `src/modules/items/controller.rs`

Controllers for handling HTTP requests:

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use crate::modules::items::model::Item;
use crate::modules::items::service;
use sqlx::sqlite::SqlitePool;

pub async fn create_item(
    State(pool): State<SqlitePool>,
    Json(payload): Json<Item>,
) -> Result<Json<Item>, StatusCode> {
    let item = service::create_item(&pool, payload.name, payload.description)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(item))
}

pub async fn list_items(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Item>>, StatusCode> {
    let items = service::list_items(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(items))
}

pub async fn get_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Item>, StatusCode> {
    let item = service::get_item(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(item))
}

pub async fn update_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<Item>,
) -> Result<StatusCode, StatusCode> {
    service::update_item(&pool, id, payload.name, payload.description)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    service::delete_item(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
```

#### `src/modules/items/routes.rs`

Routing for items module:

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::modules::items::controller::*;

pub fn create_item_routes() -> Router {
    Router::new()
        .route("/", post(create_item).get(list_items))
        .route("/:id", get(get_item).put(update_item).delete(delete_item))
}
```

### 3. Running the Application

First, run the migrations to create the necessary tables:

```sh
sqlx migrate run
```

Then, start your application:

```sh
cargo run
```

Your CRUD API will be available at `http://127.0.0.1:3005`.

### 4. Testing the API

You can test your API using tools like `curl` or Postman.

**Creating an Item**:
```sh
curl -X POST -H "Content-Type: application/json" -d '{"name": "Item 1", "description": "A sample item"}' http://127.0.0.1:3005/items
```

**Listing Items**:
```sh
curl http://127.0.0.1:3005/items
```

**Getting an Item**:
```sh
curl http://127.0.0.1:3005/items/<item_id>
```

**Updating an Item**:
```sh
curl -X PUT -H "Content-Type: application/json" -d '{"name": "Updated Item", "description": "Updated description"}' http://127.0.0.1:3005/items/<item_id>
```

**Deleting an Item**:
```sh
curl -X DELETE http://127.0.0.1:3005/items/<item_id>
```

By structuring your Rust project this way, you achieve a clean separation of concerns, making the code more maintainable and scalable, similar to the structure of a NestJS application.