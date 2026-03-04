use crate::service_trait::ServiceTrait;
use entity::product_image;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbConn, DbErr, EntityTrait, PrimaryKeyTrait,
};

pub struct ProductImageService<'a>(pub &'a DatabaseConnection);

impl ServiceTrait for ProductImageService<'_> {
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
        self.0
    }

    fn new_active_model_ex_from_id<U>(id: U) -> <Self::Entity as EntityTrait>::ActiveModelEx
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        let (pid, iid) = id.into();
        product_image::ActiveModel::builder()
            .set_product_id(pid)
            .set_image_id(iid)
    }

    fn insert_active_model_ex(
        am: <Self::Entity as EntityTrait>::ActiveModelEx,
        db: &DbConn,
    ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> + Send {
        am.insert(db)
    }

    fn update_active_model_ex(
        am: <Self::Entity as EntityTrait>::ActiveModelEx,
        db: &DbConn,
    ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> + Send {
        am.update(db)
    }
}
