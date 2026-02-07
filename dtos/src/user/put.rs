use serde::Deserialize;

#[derive(Deserialize)]
struct UserPutDto {
    name: String,
    password_hash: String,
}
