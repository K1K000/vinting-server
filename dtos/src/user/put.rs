use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserPutDto {
    id: i32,
    #[serde(flatten)]
    data: super::post::UserPostDto,
}
