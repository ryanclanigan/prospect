use crate::operations::operation::BaseOperation;
use crate::primitives::scalars::float_scalar::FloatScalar;
use crate::primitives::scalars::scalar::{BaseScalar, Scalar};
use crate::primitives::scalars::string_scalar::StringScalar;
use std::f64::NAN;

pub struct AddScalar<'a> {
  pub value: &'a Scalar,
}

impl<'a> BaseOperation<Scalar> for AddScalar<'a> {
  fn apply(&self, scalar: &Scalar) -> Result<Scalar, &'static str> {
    match &self.value {
      Scalar::String(s1) => match scalar {
        Scalar::String(s2) => Ok(Scalar::String(StringScalar::of(format!(
          "{}{}",
          s1.to_value(),
          s2.to_value()
        )))),
        Scalar::Float(_) => Err("Mismatched types: String and Float"),
      },
      Scalar::Float(f1) => match scalar {
        Scalar::Float(f2) => Ok(Scalar::Float(FloatScalar::of(match (f1, f2) {
          (x, _) if x.to_value() == NAN => NAN,
          (_, y) if y.to_value() == NAN => NAN,
          (x, y) => x.to_value() + y.to_value(),
        }))),
        Scalar::String(_) => Err("Mismatched types: Float and String"),
      },
    }
  }
}

#[cfg(test)]
mod test {

  extern crate proptest;

  use super::*;
  use proptest::prelude::*;
  use std::f64::{MAX, MIN};

  proptest! {
      #[test]
      fn strings_append(string1 in ".*", string2 in ".*") {
        let sr1 = &string1;
        let s1 = AddScalar {
          value: &Scalar::String(StringScalar::of(string1.clone())),
        };
        let s2 = Scalar::String(StringScalar::of(string2.clone()));
        assert_eq!(
          format!("{}{}",string1,string2),
          match s1.apply(&s2).unwrap() {
            Scalar::String(s) => s.to_value(),
            _ => unreachable!(),
          }
        );
      }

      #[test]
      fn strings_and_float_error(string1 in ".*", f in prop::num::f64::ANY) {
        let s1 = AddScalar {
          value: &Scalar::String(StringScalar::of(string1)),
        };
        let f1 = Scalar::Float(FloatScalar::of(f));
        assert_eq!("Mismatched types: String and Float",
          match s1.apply(&f1) {
            Err(e) => e,
            _ => unreachable!()
          }
        );
      }

      #[test]
      fn floats_and_string_error(string1 in ".*", f in prop::num::f64::ANY) {
        let s1 = Scalar::String(StringScalar::of(string1));
        let f1 = AddScalar {
          value: &Scalar::Float(FloatScalar::of(f)),
        };
        assert_eq!("Mismatched types: Float and String",
          match f1.apply(&s1) {
            Err(e) => e,
            _ => unreachable!()
          }
        );
      }

      #[test]
      fn floats_add(float1 in MIN..MAX/2f64, float2 in MIN..MAX/2f64) {
        let f1 = AddScalar {
          value: &Scalar::Float(FloatScalar::of(float1)),
        };
        let f2 = Scalar::Float(FloatScalar::of(float2));
        assert_eq!(float1 + float2,
          match f1.apply(&f2).unwrap() {
            Scalar::Float(f) => f.to_value(),
            _ => unreachable!()
          }
        );
      }
  }
}
