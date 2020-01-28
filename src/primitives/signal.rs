use super::sample::Sample;
use uuid::Uuid;

#[derive(Eq, PartialEq)]
pub struct Signal {
    samples: Vec<Sample>,
    numeric: bool,
    id: Uuid,
}

impl Signal {
    pub fn of(samples: Vec<Sample>, numeric: bool) -> Self {
        Signal {
            samples,
            numeric,
            id: Uuid::new_v4(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        self.numeric
    }

    pub fn get_samples(&mut self) -> &mut Vec<Sample> {
        &mut self.samples
    }
}

impl super::item::Item for Signal {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
