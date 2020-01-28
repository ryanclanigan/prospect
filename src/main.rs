#[macro_use]
extern crate anyhow;
extern crate actix_rt;
extern crate actix_web;
extern crate chrono;
extern crate csv;
extern crate futures;
extern crate serde;
extern crate serde_json;

mod operations;
mod primitives;
mod server;
mod storage_drivers;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, Scope};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Server's up")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(server::controllers::register_controllers)
    })
    .bind("localhost:3000")?
    .run()
    .await
}
