use serde::Deserialize;

#[derive(Deserialize)]
struct ImagePut {
    pub name: String,
    pub path: String,
}
