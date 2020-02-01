use crate::operations::boundary::boundary_signal::BoundarySignal;
use crate::operations::extremes::extremes_signal::ExtremesSignal;
use crate::operations::operation::BaseOperation;
use crate::primitives::item::Item;
use crate::primitives::sample::Sample;
use crate::primitives::scalars::scalar::{BaseScalar, Scalar};
use crate::server::output::signal_responses::*;
use crate::storage::signal_serializer::SignalSerializer;
use actix_files;
use actix_multipart::Multipart;
use actix_web::{get, post, web};
use chrono::prelude::*;
use futures::StreamExt;
use plotters::prelude::*;
use std::fs;
use std::path::Path;

pub struct SignalQueries;

impl SignalQueries {
    pub fn config(config: &mut web::ServiceConfig) {
        config
            .service(get_signals)
            .service(get_signal_csv)
            .service(post_signal)
            .service(get_signal_image);
    }
}

#[get("/signal")]
async fn get_signals() -> Result<SignalsResponse, SignalError> {
    let serializer = SignalSerializer::new();
    let signals = match serializer.get_all_signal_ids() {
        Ok(s) => s,
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };
    Ok(SignalsResponse { signals })
}

#[post("/signal")]
// TODO: The uploaded file needs a new line, which is stupid. Find a way to document that
// TODO: Use threadpool to not cause bad problems
async fn post_signal(mut payload: Multipart) -> Result<SignalResponse, SignalError> {
    let mut response = SignalResponse {
        id_or_message: "No csv file found. Please upload a csv file.".to_string(),
    };
    while let Some(item) = payload.next().await {
        let field = item.unwrap();
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        if !filename.ends_with(".csv") {
            continue;
        }
        let serializer = SignalSerializer;
        let filepath = serializer.write_temp_from_bytes(filename, field).await;

        let mut temp_signal = match serializer.read_temp(filename.to_string()) {
            Ok(s) => s,
            Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
        };
        match serializer.write(&mut temp_signal) {
            Ok(_) => (),
            Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
        };
        fs::remove_file(Path::new(&filepath)).unwrap();
        response.id_or_message = temp_signal.get_id().to_string();
        return Ok(response);
    }
    Ok(response)
}

#[get("/signal/{id}/png")]
async fn get_signal_image(id: web::Path<String>) -> Result<actix_files::NamedFile, SignalError> {
    let serializer = SignalSerializer;
    let id_as_string = id.to_string();

    let mut signal = match serializer.read(&id_as_string) {
        Ok(s) => s,
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };

    if !&signal.is_numeric() {
        return Err(wrap_error_in_signal_error(
            "Sadly, displaying string signals is not currently supported, 
            as no visualization library in Rust support a string axis."
                .to_string(),
        ));
    }

    // Have to use an expanded range for the x-axis (time), so that we display everything appropriately
    let mut boundary_signal = match BoundarySignal::of(&mut signal).apply() {
        Ok(s) => s,
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };
    let boundary_samples = boundary_signal.get_samples();
    let first_sample: &Sample = match boundary_samples.first() {
        Some(s) => s,
        None => {
            return Err(wrap_error_in_signal_error(
                "Can't get get image of a signal with no data".to_string(),
            ))
        }
    };
    let last_sample: &Sample = match boundary_samples.last() {
        Some(s) => s,
        None => unreachable!(),
    };
    // Extremes are the min and max values of the signal, so they have to be the minimum and maximum areas to display
    let mut extremes_signal = match ExtremesSignal::of(&mut signal).apply() {
        Ok(s) => s,
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };
    let extremes = extremes_signal.get_samples();
    let min = match extremes[0].value {
        Scalar::Float(f) => f.to_value().to_value() - 5f64,
        Scalar::String(_) => unreachable!(),
    };
    let max = match extremes[1].value {
        Scalar::Float(f) => f.to_value().to_value() + 5f64,
        Scalar::String(_) => unreachable!(),
    };

    let filepath = format!("temp/{}.png", id_as_string);

    let data = convert_samples_to_plot_vec(signal.get_samples());
    let root = BitMapBackend::new(&filepath, (1600, 900)).into_drawing_area();
    match root.fill(&WHITE) {
        Ok(_) => (),
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };
    let root = root.margin(10, 10, 10, 10);
    let mut chart = match ChartBuilder::on(&root)
        .caption(id_as_string, ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(100)
        .build_ranged(first_sample.time..last_sample.time, min..max)
    {
        Ok(c) => c,
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };

    match chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        .draw()
    {
        Ok(_) => (),
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };

    let plot_values = convert_samples_to_plot_vec(signal.get_samples());
    match chart.draw_series(LineSeries::new(plot_values.clone(), &RED)) {
        Ok(_) => (),
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };
    match chart.draw_series(PointSeries::of_element(
        plot_values,
        5, // Size of the circle
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()); // This draws the circle
        },
    )) {
        Ok(_) => (),
        Err(e) => return Err(wrap_error_in_signal_error(e.to_string())),
    };

    Ok(actix_files::NamedFile::open(&filepath).unwrap())
}

#[get("/signal/{id}/csv")]
async fn get_signal_csv(id: web::Path<String>) -> Result<actix_files::NamedFile, SignalError> {
    let serializer = SignalSerializer;
    match serializer.get_raw_file(id.to_string()) {
        Ok(f) => Ok(actix_files::NamedFile::open(f).unwrap()),
        Err(e) => Err(SignalError {
            message: e.to_string(),
        }),
    }
}

fn wrap_error_in_signal_error(e: String) -> SignalError {
    SignalError { message: e }
}

fn convert_samples_to_plot_vec(samples: &Vec<Sample>) -> Vec<(DateTime<Utc>, f64)> {
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
