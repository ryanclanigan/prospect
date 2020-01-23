extern crate chrono;

mod operations;
mod primitives;

use chrono::prelude::*;
use chrono::Duration;
use operations::add::add_scalar::AddScalar;
use primitives::sample::Sample;
use primitives::scalars::float_scalar::FloatScalar;
use primitives::scalars::scalar::BaseScalar;
use primitives::scalars::scalar::Scalar;
use primitives::scalars::string_scalar::StringScalar;
use primitives::signal::Signal;

fn main() {
    let b = Sample {
        value: Scalar::Float(FloatScalar::of(78f64)),
        time: Utc::now(),
    };
    let c = Sample {
        value: Scalar::Float(FloatScalar::of(79f64)),
        time: b.time + Duration::seconds(60),
    };

    let signal = Signal::numeric(Box::new(vec![b, c].into_iter()));
}

fn do_math(signal: Signal) {
    //let samples: Vec<Sample> = signal.samples.collect();
}
