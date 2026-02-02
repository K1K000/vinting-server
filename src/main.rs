mod database;
mod file_server;
mod routable_file_server;
mod routes;

pub use entity; // for schema registry

use crate::{database::DatabaseFairing, file_server::FileServerFairing, routes::AllRouteFairing};

#[rocket::launch]
fn launch() -> _ {
    rocket::build()
        .attach(FileServerFairing)
        .attach(AllRouteFairing)
        .attach(DatabaseFairing)
}
