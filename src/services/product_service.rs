use entity::{product, service::ServiceTrait};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};

pub struct ProductService(DatabaseConnection);

impl ServiceTrait for ProductService {
    type Entity = product::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as product::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(product::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &DatabaseConnection {
        &self.0
    }
}
