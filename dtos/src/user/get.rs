use entity::user;
use sea_orm::prelude::DateTime;

#[derive(Debug)]
struct UserGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    pub deleted_at: Option<DateTime>,

    pub name: String,
    pub password_hash: String,
}

impl From<user::Model> for UserGetDto {
    fn from(u: user::Model) -> Self {
        Self {
            id: u.id,
            created_at: u.created_at,
            modified_at: u.modified_at,
            deleted_at: u.deleted_at,
            name: u.name,
            password_hash: u.password_hash,
        }
    }
}
