use entity::category;
use sea_orm::{ActiveValue::Set, sea_query::prelude::Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CategoryPostDto {
    name: String,
}

impl From<CategoryPostDto> for category::ActiveModelEx {
    fn from(c: CategoryPostDto) -> category::ActiveModelEx {
        category::ActiveModel::builder()
            .set_name(c.name)
            .set_deleted_at(None)
            .set_created_at(Utc::now().naive_local())
            .set_modified_at(Utc::now().naive_local())
    }
}
