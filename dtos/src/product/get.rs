use entity::product;
use sea_orm::prelude::DateTime;
use serde::Serialize;

use crate::{
    category::get::CategoryGetDto, image::get::ImageGetDto, tag::get::TagGetDto,
    user::get::UserGetDto,
};

/// Can only convert from `ModelEx` with `user`, `categories`, `tags`, and `images` loaded
#[derive(Debug, Clone, Serialize)]
pub struct ProductGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
    pub description: String,

    pub user: UserGetDto,
    pub categories: Vec<CategoryGetDto>,
    pub tags: Vec<TagGetDto>,
    pub images: Vec<ImageGetDto>,
}

impl From<product::ModelEx> for ProductGetDto {
    fn from(m: product::ModelEx) -> Self {
        // TODO: Write tests for endpoints so we don't find out in prod that these are not set
        assert!(m.categories.is_loaded());
        assert!(m.tags.is_loaded());
        assert!(m.images.is_loaded());
        assert!(m.user.is_loaded());

        let user = m.user.unwrap();

        Self {
            id: m.id,
            created_at: m.created_at,
            modified_at: m.modified_at,

            name: m.name,
            description: m.description,

            user: UserGetDto::from(user),

            // TODO: filter using service `iter_filter`
            categories: m
                .categories
                .into_iter()
                .map(CategoryGetDto::from)
                .collect::<Vec<_>>(),
            images: m
                .images
                .into_iter()
                .map(ImageGetDto::from)
                .collect::<Vec<_>>(),
            tags: m.tags.into_iter().map(TagGetDto::from).collect::<Vec<_>>(),
        }
    }
}
