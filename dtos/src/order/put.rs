use entity::order;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OrderPutDto {
    pub id: i32,
    #[serde(flatten)]
    pub data: super::post::OrderPostDto,
}

impl From<OrderPutDto> for order::ActiveModelEx {
    fn from(d: OrderPutDto) -> Self {
        order::ActiveModelEx::from(d.data).set_id(d.id)
    }
}
