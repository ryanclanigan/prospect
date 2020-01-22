use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;
use crate::primitives::signal::Signal;

pub trait BaseSignalOperation {
  fn do_timewise_operation(
    s1: Signal,
    s2: Signal,
    f: &fn(dyn BaseOperation<Sample>, Sample) -> Sample,
  ) {
  }
}
