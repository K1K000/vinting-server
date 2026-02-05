use entity::category;
use sea_orm::{prelude::*, sea_query::prelude::Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CategoryGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
}

impl From<category::Model> for CategoryGetDto {
    fn from(c: category::Model) -> Self {
        CategoryGetDto {
            id: c.id,
            created_at: c.created_at,
            modified_at: c.modified_at,
            name: c.name,
        }
    }
}
