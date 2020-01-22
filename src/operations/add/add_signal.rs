use super::add_sample::AddSample;
use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;
use crate::primitives::signal::Signal;

pub struct AddSignal {
  value: Signal,
}

impl BaseOperation<Signal> for AddSignal {
  fn apply(&self, signal: &Signal) -> Result<Signal, &'static str> {
    Err("Sample times of two samples did not match")
  }
}
