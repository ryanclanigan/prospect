mod primitives;
use primitives::sample::Sample;
use primitives::scalars::scalar::Scalar;
use primitives::scalars::string_scalar::StringScalar;

fn main() {
  let f = Sample {
    value: StringScalar::of(&"Bing".to_string()),
  };
  println!("{}", f.value.to_value());
  println!("{}", f.value.to_value());
}
