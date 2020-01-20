mod operations;
mod primitives;

use operations::add::add_scalar::AddScalar;
use primitives::sample::Sample;
use primitives::scalars::float_scalar::FloatScalar;
use primitives::scalars::scalar::BaseScalar;
use primitives::scalars::scalar::Scalar;
use primitives::scalars::string_scalar::StringScalar;
use primitives::signal::Signal;

fn main() {
  let f = Sample {
    value: Scalar::String(StringScalar::of("Bing".to_string())),
  };
  let b = Sample {
    value: Scalar::Float(FloatScalar::of(78f64)),
  };

  let vs: Vec<Sample> = vec![f, b];
  vs.into_iter().for_each(|x| beans(x.value));
}

fn beans(scalar: Scalar) {
  match scalar {
    Scalar::String(s) => println!("Value in scalar: {}", s.to_value()),
    Scalar::Float(f) => println!("Value in scalar: {}", f.to_value()),
  }
}
