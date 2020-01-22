extern crate chrono;

use super::scalars::scalar::Scalar;
use chrono::prelude::*;

pub struct Sample {
  pub value: Scalar,
  pub time: DateTime<Utc>,
}
