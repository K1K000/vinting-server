use entity::product;
use sea_orm::sea_query::prelude::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProductPostDto {
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub order_id: Option<i32>,
}

impl From<ProductPostDto> for product::ActiveModelEx {
    fn from(p: ProductPostDto) -> product::ActiveModelEx {
        product::ActiveModel::builder()
            .set_deleted_at(None)
            .set_created_at(Utc::now().naive_local())
            .set_modified_at(Utc::now().naive_local())
            .set_name(p.name)
            .set_description(p.description)
            .set_user_id(p.user_id)
            .set_order_id(p.order_id)
    }
}
