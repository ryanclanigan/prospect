use crate::server::output::operations_responses::*;
use crate::server::output::signal_responses::SignalResponse;
use actix_web::error::Result;
use actix_web::{get, web, HttpResponse, Responder};

pub struct OperationQueries;

impl OperationQueries {
    pub fn config(config: &mut web::ServiceConfig) {
        config.service(get_operations).service(do_operation);
    }
}

#[get("/operations")]
async fn get_operations() -> impl Responder {
    OperationsResponse {
        operations: vec!["Add".to_string()],
    }
}

#[get("/operations/{operation}")]
async fn do_operation(operation: web::Path<String>) -> Result<SignalResponse, OperationError> {
    unimplemented!()
}
