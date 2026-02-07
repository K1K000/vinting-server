use entity::product;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProductPutDto {
    pub id: i32,
    #[serde(flatten)]
    pub data: super::post::ProductPostDto,
}

impl From<ProductPutDto> for product::ActiveModelEx {
    fn from(d: ProductPutDto) -> Self {
        product::ActiveModelEx::from(d.data).set_id(d.id)
    }
}
