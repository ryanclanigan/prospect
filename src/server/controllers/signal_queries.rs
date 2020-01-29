use crate::server::output::signal_responses::*;
use crate::storage::signal_serializer::SignalSerializer;
use actix_web::{get, post, web, HttpResponse, Responder};

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
async fn get_signals() -> Result<SignalsResponse, SignalError> {
    let serializer = SignalSerializer::new();
    let signals = match serializer.get_all_signal_ids() {
        Ok(s) => s,
        Err(_) => {
            return Err(SignalError {
                message: "Encountered issue while reading files",
            })
        }
    };
    Ok(SignalsResponse { signals })
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
