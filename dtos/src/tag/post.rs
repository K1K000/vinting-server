use entity::tag;
use sea_orm::sea_query::prelude::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
struct TagPostDto {
    name: String,
}

impl From<TagPostDto> for tag::ActiveModelEx {
    fn from(t: TagPostDto) -> Self {
        tag::ActiveModel::builder()
            .set_deleted_at(None)
            .set_created_at(Utc::now().naive_local())
            .set_modified_at(Utc::now().naive_local())
            .set_name(t.name)
    }
}
