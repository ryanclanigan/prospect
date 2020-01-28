#[macro_use]
extern crate anyhow;

mod operations;
mod primitives;
mod server;
mod storage;
mod storage_drivers;

use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};

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
