#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use rocket::mtls::Certificate;
use rocket::fs::{FileServer, relative};

#[get("/")]
fn mutual(cert: Certificate<'_>) -> String {
    format!("Hello! Here's what we know: [{}] {}", cert.serial(), cert.subject())
}

// #[get("/", rank = 2)]
// fn hello() -> &'static str {
//     "Hello, world!"
// }

// If we wanted or needed to serve files manually, we'd use `NamedFile`. Always
// prefer to use `FileServer`!
mod manual {
    use std::path::{PathBuf, Path};
    use rocket::fs::NamedFile;

    #[rocket::get("/second/<path..>")]
    pub async fn second(path: PathBuf) -> Option<NamedFile> {
        let mut path = Path::new(super::relative!("static")).join(path);
        if path.is_dir() {
            path.push("index.html");
        }

        NamedFile::open(path).await.ok()
    }
}


#[rocket::launch]
fn rocket() -> _ {
    // See `Rocket.toml` and `Cargo.toml` for TLS configuration.
    // Run `./private/gen_certs.sh` to generate a CA and key pairs.
    rocket::build()
        .mount("/", routes![mutual])
        .mount("/", rocket::routes![manual::second])
        .mount("/", FileServer::from(relative!("static")))
}