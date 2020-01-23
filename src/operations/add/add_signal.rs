use super::add_sample::AddSample;
use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;
use crate::primitives::signal::Signal;

pub struct AddSignal<'a> {
    args: Box<[&'a mut Signal]>,
}

impl<'a> AddSignal<'a> {
    pub fn of(signal1: &'a mut Signal, signal2: &'a mut Signal) -> Self {
        AddSignal {
            args: Box::new([signal1, signal2]),
        }
    }
}

// impl<'a> BaseOperation<Signal> for AddSignal<'a> {
//   fn get_op(&self) -> Box<dyn Fn(&Box<[&Signal]>) -> Result<Signal, &'static str>> {
//     Box::new(|args: &Box<[&Signal]>| -> Result<Signal, &'static str> {
//       let signal1 = args[0];
//       let signal2 = args[1];
//       if (signal1.is_numeric() != signal2.is_numeric()) {
//         return Err("Signals are of different types");
//       }
//       let mut result: Vec<Sample> = Vec::default();
//       signal1.get_samples().next();
//       Err("eee")
//     })
//   }
// }
