use super::sample::Sample;
use std::iter;

pub struct Signal {
    samples: Box<dyn Iterator<Item = Sample>>,
    numeric: bool,
}

impl Signal {
    pub fn empty_numeric() -> Self {
        Signal {
            samples: Box::new(iter::empty()),
            numeric: true,
        }
    }

    pub fn empty_non_numeric() -> Self {
        Signal {
            samples: Box::new(iter::empty()),
            numeric: false,
        }
    }

    pub fn numeric(s: Box<dyn Iterator<Item = Sample>>) -> Self {
        Signal {
            samples: s,
            numeric: true,
        }
    }

    pub fn non_numeric(s: Box<dyn Iterator<Item = Sample>>) -> Self {
        Signal {
            samples: s,
            numeric: false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        self.numeric
    }

    pub fn get_samples(&mut self) -> &mut Box<dyn Iterator<Item = Sample>> {
        &mut self.samples
    }
}
