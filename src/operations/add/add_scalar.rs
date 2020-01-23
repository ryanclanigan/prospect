use crate::operations::operation::BaseOperation;
use crate::primitives::scalars::float_scalar::FloatScalar;
use crate::primitives::scalars::scalar::{BaseScalar, Scalar};
use crate::primitives::scalars::string_scalar::StringScalar;
use std::f64::NAN;

pub struct AddScalar<'a> {
    args: Box<[&'a Scalar]>,
}

impl<'a> AddScalar<'a> {
    pub fn of(scalar1: &'a Scalar, scalar2: &'a Scalar) -> Self {
        AddScalar {
            args: Box::new([scalar1, scalar2]),
        }
    }
}

impl<'a> BaseOperation<Scalar> for AddScalar<'a> {
    fn get_op(&self) -> Box<dyn Fn(&Box<[&Scalar]>) -> Result<Scalar, &'static str>> {
        return Box::new(|args: &Box<[&Scalar]>| -> Result<Scalar, &'static str> {
            let scalar1 = args[0];
            let scalar2 = args[1];
            match scalar1 {
                Scalar::String(s1) => match scalar2 {
                    Scalar::String(s2) => Ok(Scalar::String(StringScalar::of(format!(
                        "{}{}",
                        s1.to_value(),
                        s2.to_value()
                    )))),
                    Scalar::Float(_) => Err("Mismatched types: String and Float"),
                },
                Scalar::Float(f1) => match scalar2 {
                    Scalar::Float(f2) => Ok(Scalar::Float(FloatScalar::of(match (f1, f2) {
                        (x, _) if x.to_value() == NAN => NAN,
                        (_, y) if y.to_value() == NAN => NAN,
                        (x, y) => x.to_value() + y.to_value(),
                    }))),
                    Scalar::String(_) => Err("Mismatched types: Float and String"),
                },
            }
        });
    }

    fn get_args(&self) -> &Box<[&Scalar]> {
        &self.args
    }

    fn apply(&self) -> Result<Scalar, &'static str> {
        self.get_op()(self.get_args())
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
          let s1 = Scalar::String(StringScalar::of(string1.clone()));
          let s2 = Scalar::String(StringScalar::of(string2.clone()));
          let op = AddScalar::of(&s1,&s2);
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
          let s2 = Scalar::Float(FloatScalar::of(f));
          let op = AddScalar::of(&s1,&s2);
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
          let s1 = Scalar::Float(FloatScalar::of(f));
          let op = AddScalar::of(&s1,&s2);
          assert_eq!("Mismatched types: Float and String",
            match op.apply() {
              Err(e) => e,
              _ => unreachable!()
            }
          );
        }

        #[test]
        fn floats_add(float1 in MIN..MAX/2f64, float2 in MIN..MAX/2f64) {
          let s1 = Scalar::Float(FloatScalar::of(float1));
          let s2 = Scalar::Float(FloatScalar::of(float2));
          let op = AddScalar::of(&s1, &s2);
          assert_eq!(float1 + float2,
            match op.apply().unwrap() {
              Scalar::Float(f) => f.to_value(),
              _ => unreachable!()
            }
          );
        }
    }
}
