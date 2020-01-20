use super::sample::Sample;
use std::iter;

pub struct Signal {
  samples: Box<dyn Iterator<Item = Sample>>,
}

impl Signal {
  fn empty() -> Self {
    Signal {
      samples: Box::new(iter::empty()),
    }
  }

  fn getSamples(&self) -> &Box<dyn Iterator<Item = Sample>> {
    return &self.samples;
  }
}
