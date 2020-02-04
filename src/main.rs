#[macro_use]
extern crate anyhow;

mod drivers;
mod operations;
mod primitives;
mod server;
mod storage;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use env_logger;
use storage::signal_serializer::SignalSerializer;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Server's up")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    match SignalSerializer::init_once() {
        Ok(_) => (),
        Err(e) => panic!("Dir creation error: {}", e),
    };

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .configure(server::controllers::register_controllers)
    })
    .bind("localhost:3000")?
    .run()
    .await
}
