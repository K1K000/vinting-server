use entity::category;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryPostDto {
    pub name: String,
}

impl From<CategoryPostDto> for category::ActiveModelEx {
    fn from(c: CategoryPostDto) -> category::ActiveModelEx {
        category::ActiveModel::builder().set_name(c.name)
    }
}

crate::active_actions!(category::ActiveModelEx);
