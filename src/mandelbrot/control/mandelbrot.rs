
use image::{Rgb, RgbImage};
use log::info;
use rayon::prelude::*;
use crate::mandelbrot::control::{iteration_function};
use crate::images::color_palette::ColorPalette;
use crate::mandelbrot::entity::fractal::Fractal;

pub fn generate_mandelbrot(fractal: Fractal) -> RgbImage {
    let palette = ColorPalette::new(fractal.max_iterations);

    let start = std::time::Instant::now();
    let mut img = RgbImage::new(fractal.width, fractal.height);
    let max_iterations = fractal.max_iterations;

    // Define a function for color mapping
    let map_value_to_color = |value: u32| {
        if value == max_iterations {
            Rgb([0, 0, 255]) // Black color for points in the Mandelbrot set
        } else {
            let color_index: usize = (value % (palette.get_palette().len() as u32)) as usize;
            palette.get_palette()[color_index]
        }
    };

    let rows: Vec<_> = (0..fractal.height)
        .into_par_iter()
        .map(|y| {
            (0..fractal.width).map(move |x| {
                let x = (f64::from(x) / f64::from(fractal.width)) * (fractal.c1 - fractal.c0) + fractal.c0;
                let y = (f64::from(y) / f64::from(fractal.height)) * (fractal.c1i - fractal.c0i) + fractal.c0i;

                let value = iteration_function::mandelbrot(x, y, max_iterations);
                map_value_to_color(value)
            }).collect::<Vec<_>>()
        }).collect();

    for (y, row) in rows.into_iter().enumerate() {
        for (x, pixel) in row.into_iter().enumerate() {
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    info!("Mandelbrot set generated in {} ms", start.elapsed().as_millis());
    img
}

