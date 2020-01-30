#[macro_use]
extern crate anyhow;

mod operations;
mod primitives;
mod server;
mod storage;
mod storage_drivers;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use storage::signal_serializer::SignalSerializer;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Server's up")
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     match SignalSerializer::init_once() {
//         Ok(_) => (),
//         Err(e) => panic!("Dir creation error: {}", e),
//     };
//     HttpServer::new(|| {
//         App::new()
//             .service(index)
//             .configure(server::controllers::register_controllers)
//     })
//     .bind("localhost:3000")?
//     .run()
//     .await
// }

use plotlib::page::Page;
use plotlib::scatter;
use plotlib::scatter::Scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::ContinuousView;

use primitives::scalars::F64::F64;

fn main() {
    // // Scatter plots expect a list of pairs
    // let data1 = [
    //     (-3.0, 2.3),
    //     (-1.6, 5.3),
    //     (0.3, 0.7),
    //     (4.3, -1.4),
    //     (6.4, 4.3),
    //     (8.5, 3.7),
    // ];

    // // We create our scatter plot from the data
    // let s1 = Scatter::from_slice(&data1).style(
    //     scatter::Style::new()
    //         .marker(Marker::Square) // setting the marker to be a square
    //         .colour("#DD3355"),
    // ); // and a custom colour

    // // We can plot multiple data sets in the same view
    // let data2 = [(-1.4, 2.5), (7.2, -0.3)];
    // let s2 = Scatter::from_slice(&data2).style(
    //     scatter::Style::new() // uses the default marker
    //         .colour("#35C788"),
    // ); // and a different colour

    // // The 'view' describes what set of data is drawn
    // let v = ContinuousView::new()
    //     .add(&s1)
    //     .add(&s2)
    //     .x_range(-5., 10.)
    //     .y_range(-2., 6.)
    //     .x_label("Some varying variable")
    //     .y_label("The response of something");

    // // A page with a single view is then saved to an SVG file
    // Page::single(&v).save("scatter.svg");

    let f1 = F64::of(1f64);
    let f2 = F64::of(2f64);
    assert!(f1 < f2);
    assert!(f2 > f1);
}
