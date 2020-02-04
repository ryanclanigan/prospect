use crate::drivers::visualization::graph_generator::GraphGenerator;
use crate::primitives::item::Item;
use crate::primitives::sample::Sample;
use crate::primitives::scalars::scalar::{BaseScalar, Scalar};
use crate::server::output::signal_responses::*;
use crate::storage::signal_serializer::SignalSerializer;
use actix_files;
use actix_multipart::Multipart;
use actix_web::{get, post, web};
use chrono::prelude::*;
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
            .service(get_signal_image);
    }
}

#[get("/signal")]
async fn get_signals() -> Result<SignalsResponse, SignalError> {
    let serializer = SignalSerializer::new();
    let signals = match serializer.get_all_signal_ids() {
        Ok(s) => s,
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };
    Ok(SignalsResponse { signals })
}

#[post("/signal")]
// TODO: The uploaded file needs a new line, which is stupid. Find a way to document that. Do it in README?
async fn post_signal(mut payload: Multipart) -> Result<SignalResponse, SignalError> {
    let mut response = SignalResponse {
        id_or_message: "No csv file found. Please upload a csv file.".to_string(),
    };
    while let Some(item) = payload.next().await {
        let field = match item {
            Ok(f) => f,
            Err(e) => {
                return Err(wrap_error_in_signal_error(format!(
                    "Error trying to unwrap payload. Source error: {}",
                    e.to_string()
                )))
            }
        };
        let content_type = match field.content_disposition() {
            Some(ct) => ct,
            None => return Err(wrap_error_in_signal_error(
                "No content dispostion found in payload. Please make sure you're sending a csv file".to_string())),
        };
        let filename = match content_type.get_filename() {
            Some(f) => f,
            None => return Err(wrap_error_in_signal_error("No file name found in the content type. Please make sure you're sending a csv file".to_string()))
        };
        if !filename.ends_with(".csv") {
            continue;
        }
        let serializer = SignalSerializer;
        let filepath = match serializer.write_temp_from_bytes(filename, field).await {
            Ok(f) => f,
            Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
        };

        let mut temp_signal = match serializer.read_temp(filename.to_string()) {
            Ok(s) => s,
            Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
        };
        match serializer.write(&mut temp_signal) {
            Ok(_) => (),
            Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
        };
        if let Err(e) = fs::remove_file(Path::new(&filepath)) {
            return Err(wrap_error_in_signal_error(e.to_string()));
        };
        response.id_or_message = temp_signal.get_id().to_string();
        return Ok(response);
    }
    Ok(response)
}

#[get("/signal/{id}/png")]
async fn get_signal_image(id: web::Path<String>) -> Result<actix_files::NamedFile, SignalError> {
    let serializer = SignalSerializer;
    let id_as_string = id.to_string();

    let signal = match serializer.read(&id_as_string) {
        Ok(s) => s,
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };

    if !&signal.is_numeric() {
        return Err(wrap_error_in_signal_error(
            "Sadly, displaying string signals is not currently supported, 
            as no visualization library in Rust support a string axis."
                .to_string(),
        ));
    }
    let filepath = format!("temp/{}.png", id_as_string);

    let generator = GraphGenerator;

    if let Err(e) = generator.draw_signal(signal, &filepath, id_as_string) {
        return Err(wrap_error_in_signal_error_with_prefix(
            "Error occured while generating image: ".to_string(),
            e.to_string(),
        ));
    };

    match actix_files::NamedFile::open(&filepath) {
        Ok(f) => Ok(f),
        Err(e) => Err(wrap_error_in_signal_error_with_prefix(
            "Could not find png file after creating it".to_string(),
            e.to_string(),
        )),
    }
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

fn wrap_error_in_signal_error(e: String) -> SignalError {
    SignalError { message: e }
}

fn wrap_error_in_signal_error_with_prefix(prefix: String, e: String) -> SignalError {
    wrap_error_in_signal_error(format!("{}: {}", prefix, e))
}
