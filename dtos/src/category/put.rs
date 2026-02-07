use entity::category;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryPutDto {
    pub id: i32,
    #[serde(flatten)]
    pub data: super::post::CategoryPostDto,
}

impl From<CategoryPutDto> for category::ActiveModelEx {
    fn from(d: CategoryPutDto) -> Self {
        category::ActiveModelEx::from(d.data).set_id(d.id)
    }
}
