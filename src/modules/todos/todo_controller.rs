use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

use sqlx::sqlite::SqlitePool;
use crate::modules::todos::todo_dto::{CreateItemDto, UpdateItemDto};
use crate::modules::todos::todo_entity::Item;
use crate::modules::todos::todo_service;

pub async fn create_item(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateItemDto>,
) -> Result<Json<Item>, StatusCode> {
    let item = todo_service::create_item(&pool, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(item))
}

pub async fn list_items(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Item>>, StatusCode> {
    let items = todo_service::list_items(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(items))
}

pub async fn get_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Item>, StatusCode> {
    let item = todo_service::get_item(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(item))
}

pub async fn update_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateItemDto>,
) -> Result<StatusCode, StatusCode> {
    todo_service::update_item(&pool, id, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    todo_service::delete_item(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
