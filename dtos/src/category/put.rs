use serde::Deserialize;

#[derive(Deserialize)]
pub struct CategoryPut {
    name: String,
}
