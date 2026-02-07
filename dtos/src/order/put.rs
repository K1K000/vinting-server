use serde::Deserialize;

#[derive(Deserialize)]
struct OrderPutDto {
    pub user_id: i32,
}
