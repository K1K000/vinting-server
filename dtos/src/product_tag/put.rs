use serde::Deserialize;

#[derive(Deserialize)]
struct ProductTagPutDto {
    product_id: i32,
    tag_id: i32,
}
