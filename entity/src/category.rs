use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    pub deleted_at: Option<DateTime>,

    pub name: String,

    #[sea_orm(has_many)]
    pub product_categories: HasMany<super::product_category::Entity>,
    #[sea_orm(has_many, via = "product_category")]
    pub products: HasMany<super::product::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
