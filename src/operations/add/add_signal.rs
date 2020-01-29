use super::add_sample::AddSample;
use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;
use crate::primitives::signal::Signal;

pub struct AddSignal<'a> {
    signal1: &'a mut Signal,
    signal2: &'a mut Signal,
}

impl<'a> AddSignal<'a> {
    pub fn of(signal1: &'a mut Signal, signal2: &'a mut Signal) -> Self {
        AddSignal { signal1, signal2 }
    }
}

impl<'a> BaseOperation for AddSignal<'a> {
    type Primitive = Signal;

    fn apply(&mut self) -> Result<Signal, &'static str> {
        if self.signal1.is_numeric() != self.signal2.is_numeric() {
            return Err("Signals are of different types");
        }

        let mut new_samples: Vec<Sample> = Vec::default();
        let mut iter1 = self.signal1.get_samples().iter();
        let mut iter2 = self.signal2.get_samples().iter();
        let mut sample1 = iter1.next();
        let mut sample2 = iter2.next();
        let mut change1 = false;
        let mut change2 = false;
        loop {
            if change1 {
                sample1 = iter1.next();
                change1 = false;
            }
            if change2 {
                sample2 = iter2.next();
                change2 = false;
            }
            if sample1.is_none() && sample2.is_none() {
                break;
            }
            match &sample1 {
                Some(s1) => match &sample2 {
                    Some(s2) => {
                        if s1.is_keyed_before(s2) {
                            change1 = true;
                            new_samples.push((*s1).clone());
                        } else if s1.is_keyed_after(s2) {
                            change2 = true;
                            new_samples.push((*s2).clone());
                        } else {
                            match AddSample::of(s1, s2).apply() {
                                Ok(s) => {
                                    change1 = true;
                                    change2 = true;
                                    new_samples.push(s);
                                }
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    None => {
                        change1 = true;
                        new_samples.push((*s1).clone());
                    }
                },
                None => match &sample2 {
                    Some(s2) => {
                        change2 = true;
                        new_samples.push((*s2).clone());
                    }
                    None => break,
                },
            };
        }
        Ok(Signal::of(new_samples, self.signal1.is_numeric()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::scalars::float_scalar::FloatScalar;
    use crate::primitives::scalars::scalar::*;
    use crate::primitives::scalars::string_scalar::StringScalar;
    use crate::primitives::scalars::F64::F64;
    use chrono::prelude::*;
    use chrono::Duration;
    use proptest::prelude::*;
    use std::f64::MAX;

    proptest! {
        #[test]
        fn add_strings(string1 in ".*", string2 in ".*", string3 in ".*") {
          let now = Utc::now();
          let early = now - Duration::days(1);
          let later = now + Duration::days(1);
          let s1 = Sample::of(Scalar::String(StringScalar::of(string1.clone())), early);
          let s2 = Sample::of(Scalar::String(StringScalar::of(string2.clone())), now);
          let s3 = Sample::of(Scalar::String(StringScalar::of(string3.clone())), later);
          let samples1 = vec![s1.clone(),s2.clone(),s3.clone()];
          let samples2 = vec![s2.clone()];
          let mut expected_string_2 = string2.clone();
          expected_string_2.push_str(&string2);
          let expected_samples = vec![s1.clone(), Sample::of(Scalar::String(StringScalar::of(expected_string_2)), now), s3.clone()];
          let mut signal1 = Signal::of(samples1.clone(), false);
          let mut signal2 = Signal::of(samples2.clone(), false);
          let mut result_signal = AddSignal::of(&mut signal1, &mut signal2).apply().unwrap();
          assert!(expected_samples == *result_signal.get_samples());
        }

        #[test]
        fn add_floats(float1 in 0f64..MAX/2f64, float2 in 0f64..MAX/2f64, float3 in 0f64..MAX/2f64) {
          let now = Utc::now();
          let early = now - Duration::days(1);
          let later = now + Duration::days(1);
          let s1 = Sample::of(Scalar::Float(FloatScalar::of(F64::of(float1))), early);
          let s2 = Sample::of(Scalar::Float(FloatScalar::of(F64::of(float2))), now);
          let s3 = Sample::of(Scalar::Float(FloatScalar::of(F64::of(float3))), later);
          let samples1 = vec![s1.clone(), s2.clone(), s3.clone()];
          let samples2 = vec![s2.clone()];
          let expected_float_2 = float2 + float2;
          let expected_samples = vec![s1.clone(), Sample::of(Scalar::Float(FloatScalar::of(F64::of(expected_float_2))), now), s3.clone()];
          let mut signal1 = Signal::of(samples1.clone(), true);
          let mut signal2 = Signal::of(samples2.clone(), true);
          let mut result_signal = AddSignal::of(&mut signal1, &mut signal2).apply().unwrap();
          assert!(expected_samples == *result_signal.get_samples());
        }
    }
}
