use entity::product_tag;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProductCategoryPutDto {
    pub product_id: i32,
    pub category_id: i32,
}
