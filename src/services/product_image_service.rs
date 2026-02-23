use entity::{product_image, service::ServiceTrait};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};

pub struct ProductImageService(DatabaseConnection);

impl ServiceTrait for ProductImageService {
    type Entity = product_image::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as product_image::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(product_image::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &DatabaseConnection {
        &self.0
    }
}
