use crate::service_trait::ServiceTrait;
use entity::product_category;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};

pub struct ProductCategoryService(DatabaseConnection);

impl ServiceTrait for ProductCategoryService {
    type Entity = product_category::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as product_category::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(product_category::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &DatabaseConnection {
        &self.0
    }
}
