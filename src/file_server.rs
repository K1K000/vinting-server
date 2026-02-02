use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    fs::{FileServer, Options, relative},
    routes,
};
use tokio::fs::try_exists;

use crate::routable_file_server::get_root_regardless;

pub struct FileServerFairing;

#[async_trait]
impl Fairing for FileServerFairing {
    fn info(&self) -> Info {
        Info {
            name: "Fairing for static files",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        match try_exists("./web/").await.unwrap_or(false) {
            true => Ok(r
                .mount(
                    "/",
                    FileServer::new(relative!("/web"), Options::default() | Options::Missing),
                )
                .mount("/", routes![get_root_regardless])),
            false => {
                log::error!("The 'web' directory is not present, nothing to host");
                Err(r)
            }
        }
    }
}
