use entity::product_image;
use sea_orm::sea_query::prelude::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProductImagePostDto {
    pub product_id: i32,
    pub image_id: i32,
}

impl From<ProductImagePostDto> for product_image::ActiveModelEx {
    fn from(pi: ProductImagePostDto) -> Self {
        product_image::ActiveModel::builder()
            .set_deleted_at(None)
            .set_created_at(Utc::now().naive_local())
            .set_modified_at(Utc::now().naive_local())
            .set_image_id(pi.image_id)
            .set_product_id(pi.product_id)
    }
}
