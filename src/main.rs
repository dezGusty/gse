#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use rocket::fs::{FileServer, relative};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
}