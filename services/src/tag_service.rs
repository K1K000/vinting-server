use crate::service_trait::ServiceTrait;
use entity::tag;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};

pub struct TagService<'a>(&'a DatabaseConnection);

impl ServiceTrait for TagService<'_> {
    type Entity = tag::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as tag::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(tag::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &DatabaseConnection {
        self.0
    }
}
