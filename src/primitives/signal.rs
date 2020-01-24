use super::sample::Sample;

#[derive(Eq, PartialEq)]
pub struct Signal {
    samples: Vec<Sample>,
    numeric: bool,
}

impl Signal {
    pub fn of(samples: Vec<Sample>, numeric: bool) -> Self {
        Signal { samples, numeric }
    }

    pub fn is_numeric(&self) -> bool {
        self.numeric
    }

    pub fn get_samples(&mut self) -> &mut Vec<Sample> {
        &mut self.samples
    }
}
