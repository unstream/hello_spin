
use image::{ImageBuffer, Luma, Rgb};
use log::info;
use crate::images::color_palette::ColorPalette;

pub fn convert_gray16_to_rgb(image: ImageBuffer<Luma<u16>, Vec<u16>>, black_threshold: u32, palette: &ColorPalette) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let mut rgb_image = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
        let color = map_value_to_color(pixel[0] as u32, black_threshold, palette);
        *rgb_image.get_pixel_mut(x, y) = color;
    }

    rgb_image
}


/// Maps a given value to a color based on the provided color palette.
///
/// This function is used to map the number of iterations taken for a point to escape the Mandelbrot set
/// to a color that can be used to visualize the set. Points that are in the set (i.e., did not escape even after
/// `max_iterations` iterations) are colored black.
///
/// # Arguments
///
/// * `value` - The number of iterations it took for a point to escape the Mandelbrot set. If the point is in the set,
/// this should be equal to `max_iterations`.
///
/// * `max_iterations` - The maximum number of iterations to consider for a point to be in the Mandelbrot set.
///
/// * `palette` - A reference to a `ColorPalette` object that defines the colors to use for points that are not in the set.
///
/// # Returns
///
/// * `Rgb<u8>` - An RGB color that represents the number of iterations it took for a point to escape the Mandelbrot set.
/// Points that are in the set are represented by the color black.
///
/// # Example
///
/// ```
/// let palette = ColorPalette::new(256);
/// let color = map_value_to_color(50, 256, &palette);
/// ```
pub fn map_value_to_color(value: u32, black_threshold: u32, palette: &ColorPalette) -> Rgb<u8> {
    if value > black_threshold {
        info!("Value {}", value);
        Rgb([0, 0, 0]) // Black color for points in the Mandelbrot set
    } else {
        let color_index: usize = (value % (palette.get_palette().len() as u32)) as usize;
        palette.get_palette()[color_index]
    }
}



