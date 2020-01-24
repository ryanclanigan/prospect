use crate::operations::operation::BaseOperation;
use crate::primitives::scalars::float_scalar::*;
use crate::primitives::scalars::scalar::{BaseScalar, Scalar};
use crate::primitives::scalars::string_scalar::StringScalar;

pub struct AddScalar<'a> {
    scalar1: &'a Scalar,
    scalar2: &'a Scalar,
}

impl<'a> AddScalar<'a> {
    pub fn of(scalar1: &'a Scalar, scalar2: &'a Scalar) -> Self {
        AddScalar { scalar1, scalar2 }
    }
}

impl<'a> BaseOperation<Scalar> for AddScalar<'a> {
    fn apply(&mut self) -> Result<Scalar, &'static str> {
        match self.scalar1 {
            Scalar::String(s1) => match self.scalar2 {
                Scalar::String(s2) => Ok(Scalar::String(StringScalar::of(format!(
                    "{}{}",
                    s1.to_value(),
                    s2.to_value()
                )))),
                Scalar::Float(_) => Err("Mismatched types: String and Float"),
            },
            Scalar::Float(f1) => match self.scalar2 {
                Scalar::Float(f2) => Ok(Scalar::Float(FloatScalar::of(
                    f1.to_value() + f2.to_value(),
                ))),
                Scalar::String(_) => Err("Mismatched types: Float and String"),
            },
        }
    }
}

#[cfg(test)]
mod test {

    extern crate proptest;

    use super::*;
    use crate::primitives::scalars::F64::*;
    use proptest::prelude::*;
    use std::f64::{MAX, MIN};

    proptest! {
        #[test]
        fn strings_append(string1 in ".*", string2 in ".*") {
          let s1 = Scalar::String(StringScalar::of(string1.clone()));
          let s2 = Scalar::String(StringScalar::of(string2.clone()));
          let mut op = AddScalar::of(&s1,&s2);
          assert_eq!(
            format!("{}{}",string1,string2),
            match op.apply().unwrap() {
              Scalar::String(s) => s.to_value(),
              _ => unreachable!(),
            }
          );
        }

        #[test]
        fn strings_and_float_error(string1 in ".*", f in prop::num::f64::ANY) {
          let s1 = Scalar::String(StringScalar::of(string1.clone()));
          let s2 = Scalar::Float(FloatScalar::of(F64::of(f)));
          let mut op = AddScalar::of(&s1,&s2);
          assert_eq!("Mismatched types: String and Float",
            match op.apply() {
              Err(e) => e,
              _ => unreachable!()
            }
          );
        }

        #[test]
        fn floats_and_string_error(string1 in ".*", f in prop::num::f64::ANY) {
          let s2 = Scalar::String(StringScalar::of(string1.clone()));
          let s1 = Scalar::Float(FloatScalar::of(F64::of(f)));
          let mut op = AddScalar::of(&s1,&s2);
          assert_eq!("Mismatched types: Float and String",
            match op.apply() {
              Err(e) => e,
              _ => unreachable!()
            }
          );
        }

        #[test]
        fn floats_add(float1 in MIN..MAX/2f64, float2 in MIN..MAX/2f64) {
          let s1 = Scalar::Float(FloatScalar::of(F64::of(float1)));
          let s2 = Scalar::Float(FloatScalar::of(F64::of(float2)));
          let mut op = AddScalar::of(&s1, &s2);
          assert_eq!(F64::of(float1) + F64::of(float2),
            match op.apply().unwrap() {
              Scalar::Float(f) => f.to_value(),
              _ => unreachable!()
            }
          );
        }
    }
}
