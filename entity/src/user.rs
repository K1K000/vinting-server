use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    // TODO: use argon2
    // see usage at https://docs.rs/argon2/latest/argon2/
    pub password_hash: String,
    // TODO: more fields?
}

impl ActiveModelBehavior for ActiveModel {}
