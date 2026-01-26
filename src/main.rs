mod file_server;
mod routes;

use crate::{file_server::FileServerFairing, routes::AllRouteFairing};

#[rocket::launch]
fn launch() -> _ {
    rocket::build()
        .attach(FileServerFairing)
        .attach(AllRouteFairing)
}
