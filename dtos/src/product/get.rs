use entity::product;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ProductGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub order_id: Option<i32>,
}

impl From<product::Model> for ProductGetDto {
    fn from(p: product::Model) -> Self {
        Self {
            id: p.id,
            created_at: p.created_at,
            modified_at: p.modified_at,

            name: p.name,
            description: p.description,
            user_id: p.user_id,
            order_id: p.order_id,
        }
    }
}
