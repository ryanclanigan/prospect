use actix_web::{error, Error, HttpRequest, HttpResponse, Responder, Result};
use futures::future::{ready, Ready};
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct SignalsResponse {
    pub signals: Vec<String>,
}

impl Responder for SignalsResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Debug)]
pub struct SignalError {
    pub message: &'static str,
}

impl fmt::Display for SignalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::ResponseError for SignalError {}
