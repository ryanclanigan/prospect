use crate::primitives::item::Item;
use crate::server::output::signal_responses::*;
use crate::storage::signal_serializer::SignalSerializer;
use actix_files;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use std::fs;
use std::path::Path;

pub struct SignalQueries;

impl SignalQueries {
    pub fn config(config: &mut web::ServiceConfig) {
        config
            .service(get_signals)
            .service(get_signal_csv)
            .service(post_signal)
            .service(post_samples);
    }
}

#[get("/signal")]
async fn get_signals() -> Result<SignalsResponse, SignalError> {
    let serializer = SignalSerializer::new();
    let signals = match serializer.get_all_signal_ids() {
        Ok(s) => s,
        Err(_) => {
            return Err(SignalError {
                message: "Encountered issue while reading files".to_string(),
            })
        }
    };
    Ok(SignalsResponse { signals })
}

#[post("/signal")]
// TODO: The uploaded file needs a new line, which is stupid. Find a way to document that
// TODO: Use threadpool to not cause bad problems
async fn post_signal(mut payload: Multipart) -> Result<SignalResponse, SignalError> {
    let mut response = SignalResponse {
        id_or_message: "No csv file found. Please upload a csv file.".to_string(),
    };
    while let Some(item) = payload.next().await {
        let field = item.unwrap();
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        if !filename.ends_with(".csv") {
            continue;
        }
        let serializer = SignalSerializer;
        let filepath = serializer.write_temp_from_bytes(filename, field).await;

        let mut temp_signal = match serializer.read_temp(filename.to_string()) {
            Ok(s) => s,
            Err(e) => {
                return Err(SignalError {
                    message: format!("{}", e),
                })
            }
        };
        match serializer.write(&mut temp_signal) {
            Ok(_) => (),
            Err(e) => {
                return Err(SignalError {
                    message: format!("{}", e),
                })
            }
        };
        fs::remove_file(Path::new(&filepath)).unwrap();
        response.id_or_message = temp_signal.get_id().to_string();
        return Ok(response);
    }
    Ok(response)
}

#[post("/signal/{id}")]
async fn post_samples() -> impl Responder {
    HttpResponse::Ok().body("F")
}

#[get("/signal/{id}/csv")]
async fn get_signal_csv(id: web::Path<String>) -> Result<actix_files::NamedFile, SignalError> {
    let serializer = SignalSerializer;
    match serializer.get_raw_file(id.to_string()) {
        Ok(f) => Ok(actix_files::NamedFile::open(f).unwrap()),
        Err(e) => Err(SignalError {
            message: e.to_string(),
        }),
    }
}
