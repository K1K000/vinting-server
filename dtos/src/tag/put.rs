use serde::Deserialize;

#[derive(Deserialize)]
struct TagPutDto {
    name: String,
}
