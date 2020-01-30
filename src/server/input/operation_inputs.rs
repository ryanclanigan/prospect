use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignalInputs {
    // A vec of UUIDs
    pub inputs: Vec<String>,
}
