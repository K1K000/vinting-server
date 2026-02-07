use std::path::PathBuf;

use entity::image;
use sea_orm::prelude::DateTime;
use serde::Serialize;

use crate::from_models;

#[derive(Debug, Clone, Serialize)]
pub struct ImageGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub url: String,
}

from_models!(image, ImageGetDto, m, {
    let path = PathBuf::from(m.path);
    let file_name = path.file_name().expect("Malformed image path");

    ImageGetDto {
        id: m.id,
        created_at: m.created_at,
        modified_at: m.modified_at,
        // TODO: get the server url from a config or something and return an actual link
        url: format!("/img/{}", file_name.display()),
    }
});
