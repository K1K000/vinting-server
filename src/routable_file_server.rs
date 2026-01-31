use std::path::PathBuf;

use rocket::{
    fs::{NamedFile, relative},
    get,
};

/// React router does internal routing,
/// which is fine when the user is simply navigating the website,
/// however, when the user refreshes the page it will 404,
/// since the path most likely won't be found.
/// This is where this function comes in.
/// Regardless of the requested file, this will return the root index.html,
/// so react-router can do client-side routing
#[get("/<_file..>", rank = 999)]
pub async fn get_root_regardless(_file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(relative!("web/index.html")).await.ok()
}
