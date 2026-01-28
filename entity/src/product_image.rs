use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    pub deleted_at: Option<DateTime>,

    pub product_id: i32,
    #[sea_orm(belongs_to, from = "product_id", to = "id")]
    pub product: HasOne<super::product::Entity>,

    pub image_id: i32,
    #[sea_orm(belongs_to, from = "image_id", to = "id")]
    pub tag: HasOne<super::image::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
