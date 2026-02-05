use entity::product_image;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ProductImageGetDto {
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub product_id: i32,
    pub image_id: i32,
}

impl From<product_image::Model> for ProductImageGetDto {
    fn from(pc: product_image::Model) -> Self {
        Self {
            created_at: pc.created_at,
            modified_at: pc.modified_at,

            product_id: pc.product_id,
            image_id: pc.image_id,
        }
    }
}
