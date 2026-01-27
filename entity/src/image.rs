use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    // WARN: DO NOT ACCEPT THIS FROM THE USER
    // TODO: in the endpoint responsable for uploads:
    // generate random file name (probably using the file's hash) and put the image there
    // afterwards use that as the image path in the db
    pub path: String,
}

impl ActiveModelBehavior for ActiveModel {}
