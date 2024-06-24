use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize,FromRow,Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
}
