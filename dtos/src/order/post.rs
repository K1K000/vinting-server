use entity::order;
use sea_orm::sea_query::prelude::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
struct OrderPostDto {
    pub user_id: i32,
}

impl From<OrderPostDto> for order::ActiveModelEx {
    fn from(c: OrderPostDto) -> order::ActiveModelEx {
        order::ActiveModel::builder()
            .set_deleted_at(None)
            .set_created_at(Utc::now().naive_local())
            .set_modified_at(Utc::now().naive_local())
            .set_user_id(c.user_id)
    }
}
