use axum::Router;
use axum_todo_app::db::init_db;
use axum_todo_app::modules::todos::create_item_routes;

mod db;

#[tokio::main]
async fn main() {
    // Initialize database pool
    let pool = init_db().await.expect("Failed to initialize the database");

    // Create app with routes
    let app = Router::new()
        .nest("/items", create_item_routes())
        .with_state(pool);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
