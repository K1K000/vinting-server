use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    pub deleted_at: Option<DateTime>,

    pub name: String,
    // TODO: use argon2
    // see usage at https://docs.rs/argon2/latest/argon2/
    pub password_hash: String,

    #[sea_orm(has_many)]
    pub products: HasMany<super::product::Entity>,
    #[sea_orm(has_many)]
    pub orders: HasMany<super::order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
