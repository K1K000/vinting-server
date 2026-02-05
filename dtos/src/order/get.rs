use entity::order;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
struct OrderGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub user_id: i32,
}

impl From<order::Model> for OrderGetDto {
    fn from(o: order::Model) -> Self {
        OrderGetDto {
            id: o.id,
            created_at: o.created_at,
            modified_at: o.modified_at,
            user_id: o.user_id,
        }
    }
}
