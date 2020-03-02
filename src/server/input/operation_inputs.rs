use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignalInputs {
    pub inputs: Vec<String>,
}
