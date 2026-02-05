use entity::product_tag;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
struct ProductTagGetDto {
    created_at: DateTime,
    modified_at: DateTime,

    product_id: i32,
    tag_id: i32,
}

impl From<product_tag::Model> for ProductTagGetDto {
    fn from(pt: product_tag::Model) -> Self {
        Self {
            created_at: pt.created_at,
            modified_at: pt.modified_at,

            product_id: pt.product_id,
            tag_id: pt.tag_id,
        }
    }
}
