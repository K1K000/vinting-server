use crate::service_trait::ServiceTrait;
use entity::order;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};

pub struct OrderService<'a>(pub &'a DatabaseConnection);

impl ServiceTrait for OrderService<'_> {
    type Entity = order::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as order::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(order::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &DatabaseConnection {
        self.0
    }
}
