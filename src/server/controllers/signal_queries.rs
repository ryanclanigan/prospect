use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope};

pub struct SignalQueries;

impl SignalQueries {
    pub fn config(config: &mut web::ServiceConfig) {
        config
            .service(get_signals)
            .service(post_signal)
            .service(post_samples)
            .service(get_samples);
    }
}

#[get("/signal")]
async fn get_signals() -> impl Responder {
    HttpResponse::Ok().body("Hey leave")
}

#[post("/signal")]
async fn post_signal() -> impl Responder {
    HttpResponse::Ok().body("Bees")
}

#[post("/signal/id")]
async fn post_samples() -> impl Responder {
    HttpResponse::Ok().body("F")
}

#[get("/signal/{id}/samples")]
async fn get_samples() -> impl Responder {
    HttpResponse::Ok().body("G")
}
