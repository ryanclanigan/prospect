use crate::operations::add::add_signal::AddSignal;
use crate::operations::operation::BaseOperation;
use crate::primitives::item::Item;
use crate::primitives::signal::Signal;
use crate::server::input::operation_inputs::SignalInputs;
use crate::server::output::operation_responses::*;
use crate::server::output::signal_responses::SignalResponse;
use crate::storage::signal_serializer::SignalSerializer;
use actix_web::error::Result;
use actix_web::{get, web};

pub struct OperationQueries;

impl OperationQueries {
    pub fn config(config: &mut web::ServiceConfig) {
        config.service(do_operation);
    }
}

#[get("/operations/add")]
async fn do_operation(input: web::Json<SignalInputs>) -> Result<SignalResponse, OperationError> {
    let serializer = SignalSerializer;
    let mut signal1 = get_signal_input(&input.inputs[0], serializer)?;
    let mut signal2 = get_signal_input(&input.inputs[1], serializer)?;

    let mut result = match AddSignal::of(&mut signal1, &mut signal2).apply() {
        Ok(s) => s,
        Err(e) => {
            return Err(OperationError {
                message: e.to_string(),
            })
        }
    };

    match serializer.write(&mut result) {
        Err(e) => {
            return Err(OperationError {
                message: e.to_string(),
            })
        }
        Ok(_) => (),
    };
    Ok(SignalResponse {
        id_or_message: result.get_id().to_string(),
    })
}

fn get_signal_input(
    signal_id: &String,
    serializer: SignalSerializer,
) -> Result<Signal, OperationError> {
    match serializer.read(signal_id) {
        Ok(s) => Ok(s),
        Err(e) => Err(OperationError {
            message: e.to_string(),
        }),
    }
}
