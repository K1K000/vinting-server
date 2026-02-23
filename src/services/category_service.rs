use entity::{category, service::ServiceTrait};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};

pub struct CategoryService(DatabaseConnection);

impl ServiceTrait for CategoryService {
    type Entity = category::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as category::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(category::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &DatabaseConnection {
        &self.0
    }
}
