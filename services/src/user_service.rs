use crate::service_trait::ServiceTrait;
use entity::user;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};

pub struct UserService(DatabaseConnection);

impl ServiceTrait for UserService {
    type Entity = user::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as user::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(user::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &DatabaseConnection {
        &self.0
    }
}
