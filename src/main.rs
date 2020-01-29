#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate strum_macros;

mod operations;
mod primitives;
mod server;
mod storage;
mod storage_drivers;

use actix_files as fs;
use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};
use storage::signal_serializer::SignalSerializer;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Server's up")
}

#[get("/ggg")]
async fn fff() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("g.csv")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    match SignalSerializer::init_once() {
        Ok(_) => (),
        Err(e) => panic!("Dir creation error: {}", e),
    };
    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(server::controllers::register_controllers)
            .service(fff)
    })
    .bind("localhost:3000")?
    .run()
    .await
}
