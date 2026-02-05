use entity::image;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ImageGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
    pub path: String,
}

impl From<image::Model> for ImageGetDto {
    fn from(image: image::Model) -> Self {
        ImageGetDto {
            id: image.id,
            created_at: image.created_at,
            modified_at: image.modified_at,
            name: image.name,
            path: image.path,
        }
    }
}
