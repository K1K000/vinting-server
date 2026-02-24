pub use entity; // for schema registry

use vinting_server::{
    database::DatabaseFairing, file_server::FileServerFairing, routes::AllRouteFairing,
};

#[rocket::launch]
fn launch() -> _ {
    rocket::build()
        .attach(FileServerFairing)
        .attach(AllRouteFairing)
        .attach(DatabaseFairing)
}
