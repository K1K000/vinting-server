use entity::tag;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
struct TagGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
}

impl From<tag::Model> for TagGetDto {
    fn from(t: tag::Model) -> Self {
        Self {
            id: t.id,
            created_at: t.created_at,
            modified_at: t.modified_at,
            name: t.name,
        }
    }
}
