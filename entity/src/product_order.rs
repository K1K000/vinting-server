use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_image")]
pub struct Model {
    pub created_at: DateTime,
    pub modified_at: DateTime,
    pub deleted_at: Option<DateTime>,

    // composite key
    #[sea_orm(primary_key)]
    pub product_id: i32,
    #[sea_orm(primary_key)]
    pub order_id: i32,

    #[sea_orm(belongs_to, from = "product_id", to = "id")]
    pub product: HasOne<super::product::Entity>,

    #[sea_orm(belongs_to, from = "order_id", to = "id")]
    pub order: HasOne<super::order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
