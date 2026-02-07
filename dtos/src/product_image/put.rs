use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProductImagePutDto {
    pub product_id: i32,
    pub image_id: i32,
}
