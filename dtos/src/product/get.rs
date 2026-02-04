use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ProductGet {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    pub deleted_at: Option<DateTime>,

    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub order_id: Option<i32>,
}
