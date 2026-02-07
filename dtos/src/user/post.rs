use entity::user;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserPostDto {
    name: String,
    password: String,
}

impl From<UserPostDto> for user::ActiveModelEx {
    // doesn't set password_hash
    fn from(d: UserPostDto) -> Self {
        user::ActiveModel::builder().set_name(d.name)
    }
}

crate::active_actions!(user::ActiveModelEx);
