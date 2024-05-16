
use image::{ImageBuffer, Luma};
use log::info;
use rayon::prelude::*;
use crate::mandelbrot::control::{iteration_function};
use crate::mandelbrot::entity::fractal::Fractal;

/// Generate the imagedata
pub fn generate_mandelbrot_data(fractal: &Fractal) -> ImageBuffer<Luma<u16>, Vec<u16>> {

    let start = std::time::Instant::now();
    let mut img = image::ImageBuffer::new(fractal.width, fractal.height);
    let max_iterations = fractal.max_iterations;

    let rows: Vec<_> = (0..fractal.height)
        .into_par_iter()
        .map(|y| {
            (0..fractal.width).map(move |x| {
                let x = (f64::from(x) / f64::from(fractal.width)) * (fractal.c1 - fractal.c0) + fractal.c0;
                let y = (f64::from(y) / f64::from(fractal.height)) * (fractal.c1i - fractal.c0i) + fractal.c0i;
                iteration_function::mandelbrot(x, y, max_iterations)
            }).collect::<Vec<_>>()
        }).collect();

    for (y, row) in rows.into_iter().enumerate() {
        for (x, value) in row.into_iter().enumerate() {
            img.put_pixel(x as u32, y as u32, image::Luma([(value * 65536 / fractal.max_iterations) as u16]));
        }
    }
    info!("Mandelbrot set generated in {} ms", start.elapsed().as_millis());
    img
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mandelbrot::entity::fractal::Fractal;

    #[test]
    fn test_generate_mandelbrot() {
        let fractal = Fractal {
            c0: -2.0,
            c1: 1.0,
            c0i: -1.5,
            c1i: 1.5,
            width: 40,
            height: 40,
            max_iterations: 256,
        };

        let result = generate_mandelbrot_data(&fractal);

        assert_eq!(result.width(), 40);
        assert_eq!(result.height(), 40);
    }
}
