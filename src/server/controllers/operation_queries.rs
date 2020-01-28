use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope};

pub struct OperationQueries;

impl OperationQueries {
    pub fn config(config: &mut web::ServiceConfig) {
        config.service(get_operations).service(do_operation);
    }
}

#[get("/operations")]
async fn get_operations() -> impl Responder {
    HttpResponse::Ok().body("F")
}

#[get("/operations/{operation}")]
async fn do_operation() -> impl Responder {
    HttpResponse::Ok().body("F")
}

#[get("operations/{operation}/csv")]
async fn do_operation_csv() -> impl Responder {
    HttpResponse::Ok().body("P")
}
