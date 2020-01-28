use crate::server::output::operations_responses::*;
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
async fn do_operation() -> impl Responder {
    HttpResponse::Ok().body("F")
}

#[get("operations/{operation}/csv")]
async fn do_operation_csv() -> impl Responder {
    HttpResponse::Ok().body(Vec::new())
}
