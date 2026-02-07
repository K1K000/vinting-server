use entity::product_tag;
use sea_orm::sea_query::prelude::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProductTagPostDto {
    product_id: i32,
    tag_id: i32,
}

impl From<ProductTagPostDto> for product_tag::ActiveModelEx {
    fn from(pt: ProductTagPostDto) -> Self {
        product_tag::ActiveModel::builder()
            .set_deleted_at(None)
            .set_created_at(Utc::now().naive_local())
            .set_modified_at(Utc::now().naive_local())
            .set_product_id(pt.product_id)
            .set_tag_id(pt.tag_id)
    }
}
