pub mod operation_queries;
pub mod signal_queries;

use actix_web::web;

/// Registers all controllers. New controllers need to add their config method to this function
pub fn register_controllers(config: &mut web::ServiceConfig) {
    signal_queries::SignalQueries::config(config);
    operation_queries::OperationQueries::config(config);
}
