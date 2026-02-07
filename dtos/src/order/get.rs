use entity::order;
use sea_orm::prelude::DateTime;
use serde::Serialize;

use crate::from_models;

#[derive(Debug, Clone, Serialize)]
pub struct OrderGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub user_id: i32,
}

from_models!(order, OrderGetDto, m, {
    Self {
        id: m.id,
        created_at: m.created_at,
        modified_at: m.modified_at,
        user_id: m.user_id,
    }
});
