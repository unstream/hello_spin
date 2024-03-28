pub mod mandelbrot;
pub mod images;

use std::io::Cursor;
use anyhow::{anyhow};
use log::*;
use http::{Response, StatusCode};
use simple_logger::SimpleLogger;
use spin_sdk::http_component;
use crate::mandelbrot::entity::fractal::Fractal;


fn init_logger()  -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()?;
    Ok(())
}

/// A simple Spin HTTP component.
#[http_component]
fn get_mandelbrot(req: http::Request<()>) -> anyhow::Result<Response<Vec<u8>>> {
    init_logger()?;
    let query= req.uri().query().unwrap();
    let fractal = serde_qs::from_str::<Fractal>(query)?;

    let img = mandelbrot::control::mandelbrot::generate_mandelbrot(fractal);

    let start = std::time::Instant::now();
    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png).expect("Could not write image to buffer.");
    info!("Mandelbrot PNG image generated in {} ms", start.elapsed().as_millis());

    let response = Response::builder()
        .header("Content-type", "image/png")
        .status(StatusCode::OK)
        .body(buffer)
        .map_err(|e| anyhow!("Error creating response: {}", e))?;

    let result = Ok(response);
    result
}

