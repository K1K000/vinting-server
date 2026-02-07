use entity::product_category;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ProductCategoryGetDto {
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub product_id: i32,
    pub category_id: i32,
}

impl From<product_category::Model> for ProductCategoryGetDto {
    fn from(pc: product_category::Model) -> Self {
        Self {
            created_at: pc.created_at,
            modified_at: pc.modified_at,

            product_id: pc.product_id,
            category_id: pc.category_id,
        }
    }
}
