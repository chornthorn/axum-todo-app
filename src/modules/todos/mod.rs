use axum::Router;
use axum::routing::{get, post};
use sqlx::SqlitePool;
use crate::modules::todos::todo_controller::{create_item, delete_item, get_item, list_items, update_item};

pub mod todo_controller;
pub mod todo_service;
pub mod todo_entity;
pub mod todo_dto;


pub fn create_item_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", post(create_item).get(list_items))
        .route("/:id", get(get_item).put(update_item).delete(delete_item))
}