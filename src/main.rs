use crate::file_server::FileServerFairing;

mod file_server;

#[rocket::launch]
fn launch() -> _ {
    rocket::build().attach(FileServerFairing)
}
