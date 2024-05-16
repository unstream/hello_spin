use std::io::Cursor;
use anyhow::{anyhow};
use log::*;
use http::{Response, StatusCode};
use spin_sdk::http_component;
use crate::images::color_palette::ColorPalette;
use crate::images::color_utils::convert_gray16_to_rgb;
use crate::mandelbrot::entity::fractal::Fractal;
use crate::mandelbrot::control::mandelbrot::{generate_mandelbrot_data};
use crate::system::initialization::init_logger;
use image::ImageFormat::Png;

/// A simple Spin HTTP component.
#[http_component]
pub fn get_mandelbrot(req: http::Request<()>) -> anyhow::Result<Response<Vec<u8>>> {
    init_logger()?;
    let query= req.uri().query().unwrap();
    let fractal = serde_qs::from_str::<Fractal>(query)?;

    let gray_img = generate_mandelbrot_data(&fractal);
    let palette = ColorPalette::new(65536);
    let black_threshold = 65536 * (fractal.max_iterations - 1) / fractal.max_iterations;
    let img = convert_gray16_to_rgb(gray_img, black_threshold, &palette);

    let start = std::time::Instant::now();
    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), Png).expect("Could not write image to buffer.");
    info!("Mandelbrot PNG image generated in {} ms", start.elapsed().as_millis());

    let response = Response::builder()
        .header("Content-type", "image/png")
        .status(StatusCode::OK)
        .body(buffer)
        .map_err(|e| anyhow!("Error creating response: {}", e))?;
    let result = Ok(response);
    result
}
