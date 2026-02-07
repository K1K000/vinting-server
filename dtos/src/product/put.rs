use serde::Deserialize;

#[derive(Deserialize)]
struct ProductPutDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub order_id: Option<i32>,
}
