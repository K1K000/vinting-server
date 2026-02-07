use entity::user;
use sea_orm::sea_query::prelude::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
struct UserPostDto {
    name: String,
    password_hash: String,
}

impl From<UserPostDto> for user::ActiveModelEx {
    fn from(u: UserPostDto) -> Self {
        user::ActiveModel::builder()
            .set_deleted_at(None)
            .set_created_at(Utc::now().naive_local())
            .set_modified_at(Utc::now().naive_local())
            .set_name(u.name)
            .set_password_hash(u.password_hash)
    }
}
