use crate::operations::boundary::boundary_signal::BoundarySignal;
use crate::operations::extremes::extremes_signal::ExtremesSignal;
use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;
use crate::primitives::scalars::scalar::{BaseScalar, Scalar};
use crate::primitives::signal::Signal;
use anyhow::Error;
use chrono::prelude::*;
use plotters::prelude::*;

pub struct GraphGenerator;

impl GraphGenerator {
    pub fn draw_signal(
        self,
        mut signal: Signal,
        filepath: &String,
        id: String,
    ) -> Result<(), Error> {
        // Have to use an expanded range for the x-axis (time), so that we display everything appropriately
        let mut boundary_signal = BoundarySignal::of(&mut signal).apply()?;
        let boundary_samples = boundary_signal.get_samples();
        let first_sample: &Sample = match boundary_samples.first() {
            Some(s) => s,
            None => {
                return Err(anyhow!(
                    "Can't get get image of a signal with no data".to_string()
                ))
            }
        };
        let last_sample: &Sample = match boundary_samples.last() {
            Some(s) => s,
            None => unreachable!(),
        };
        // Extremes are the min and max values of the signal, so they have to be the minimum and maximum areas to display
        let mut extremes_signal = ExtremesSignal::of(&mut signal).apply()?;
        let extremes = extremes_signal.get_samples();
        let min = match extremes[0].value {
            Scalar::Float(f) => f.to_value().to_value() - 5f64,
            Scalar::String(_) => unreachable!(),
        };
        let max = match extremes[1].value {
            Scalar::Float(f) => f.to_value().to_value() + 5f64,
            Scalar::String(_) => unreachable!(),
        };

        let root = BitMapBackend::new(filepath, (1600, 900)).into_drawing_area();
        root.fill(&WHITE)?;
        let root = root.margin(10, 10, 10, 10);
        let mut chart = ChartBuilder::on(&root)
            .caption(id, ("sans-serif", 40).into_font())
            .x_label_area_size(20)
            .y_label_area_size(100)
            .build_ranged(first_sample.time..last_sample.time, min..max)?;

        chart
            .configure_mesh()
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(5)
            .y_labels(5)
            .draw()?;
        let plot_values = convert_samples_to_plot_vec(signal.get_samples());
        chart.draw_series(LineSeries::new(plot_values.clone(), &RED))?;
        chart.draw_series(PointSeries::of_element(
            plot_values,
            5, // Size of the circle
            &RED,
            &|c, s, st| {
                // This draws the circle
                return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
            },
        ))?;

        Ok(())
    }
}

fn convert_samples_to_plot_vec(samples: &[Sample]) -> Vec<(DateTime<Utc>, f64)> {
    let mut result: Vec<(DateTime<Utc>, f64)> = Vec::new();
    for sample in samples {
        result.push((
            sample.time,
            match sample.value {
                Scalar::Float(f) => f.to_value().to_value(),
                Scalar::String(_) => unreachable!(),
            },
        ));
    }
    // Workaround for boundary value adding samples for in memory data
    result.remove(result.len() - 1);
    result.remove(0);
    result
}
