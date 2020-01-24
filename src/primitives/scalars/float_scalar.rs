use super::scalar;
use super::F64::F64;

/// A scalar whose value is a string
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub struct FloatScalar {
    value: F64,
}

impl scalar::BaseScalar<F64> for FloatScalar {
    fn of(value: F64) -> Self {
        FloatScalar { value }
    }

    fn to_value(&self) -> F64 {
        self.value
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use crate::primitives::scalars::scalar::BaseScalar;
    use rand::Rng;
    use std::f64::{MAX, MIN};

    #[test]
    fn test_of_and_value() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n: f64 = rng.gen_range(MIN, MAX);
            let real_n = F64::of(n);
            assert_eq!(real_n, FloatScalar::of(real_n).to_value());
        }
    }
}
