use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateItemDto {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateItemDto {
    pub name: Option<String>,
    pub description: Option<String>,
}
