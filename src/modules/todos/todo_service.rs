use sqlx::sqlite::SqlitePool;
use uuid::Uuid;
use crate::modules::todos::todo_dto::{CreateItemDto, UpdateItemDto};
use crate::modules::todos::todo_entity::Item;

pub async fn create_item(pool: &SqlitePool, dto: CreateItemDto) -> Result<Item, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let item = Item {
        id: id.clone(),
        name: dto.name,
        description: dto.description,
    };

    sqlx::query("INSERT INTO items (id, name, description) VALUES (?, ?, ?)")
        .bind(&item.id)
        .bind(&item.name)
        .bind(&item.description)
        .execute(pool)
        .await?;

    Ok(item)
}

pub async fn list_items(pool: &SqlitePool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as("SELECT id, name, description FROM items")
        .fetch_all(pool)
        .await?;

    Ok(items)
}

pub async fn get_item(pool: &SqlitePool, id: String) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as("SELECT id, name, description FROM items WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn update_item(pool: &SqlitePool, id: String, dto: UpdateItemDto) -> Result<(), sqlx::Error> {
    let existing_item = get_item(pool, id.clone()).await?;

    let name = dto.name.unwrap_or(existing_item.name);
    let description = dto.description.unwrap_or(existing_item.description);

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
