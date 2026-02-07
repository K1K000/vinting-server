use entity::product_category;
use sea_orm::sea_query::prelude::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProductCategoryPostDto {
    pub product_id: i32,
    pub category_id: i32,
}

impl From<ProductCategoryPostDto> for product_category::ActiveModelEx {
    fn from(pc: ProductCategoryPostDto) -> product_category::ActiveModelEx {
        product_category::ActiveModel::builder()
            .set_deleted_at(None)
            .set_created_at(Utc::now().naive_local())
            .set_modified_at(Utc::now().naive_local())
            .set_product_id(pc.product_id)
            .set_category_id(pc.category_id)
    }
}
