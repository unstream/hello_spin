use image::Rgb;

/// A color palette generator based on the HSV color model
///
pub struct ColorPalette {
    palette: Vec<Rgb<u8>>,
}

impl ColorPalette {
    pub fn new(num_colors: u32) -> ColorPalette {
        let mut palette = Vec::with_capacity(num_colors as usize);
        for i in 0..num_colors {
            let h = (120.0 + 240.0 * (i as f32 / num_colors as f32).powf(1.5)) % 360.0;
            let s = 1.0;
            let v = (i as f32 / num_colors as f32).powf(0.5);
            palette.push(hsv_to_rgb(h, s, v));
        }
        ColorPalette { palette }
    }

    /// Returns the color palette
    ///
    pub fn get_palette(&self) -> &Vec<Rgb<u8>> {
        &self.palette
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_palette() {
        let palette = ColorPalette::new(1000);
        let color0 = palette.get_palette()[0];
        // 1st color is black
        assert_eq!(Rgb::from([0, 0, 0]), color0);
        let color999 = palette.get_palette()[999];
        // last color is red
        assert!(250 < color999.0[0]);
    }
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> Rgb<u8> {
    let mut color: Rgb<u8> = Rgb([0,0,0]);
    let hi = (hue / 60.0).floor() % 6.0;
    let f = (hue / 60.0) - (hue / 60.0).floor();
    let p = value * (1.0 - saturation);
    let q = value * (1.0 - f * saturation);
    let t = value * (1.0 - (1.0 - f) * saturation);

    match hi as i32 {
        0 => color = Rgb([(value * 255.0) as u8, (t * 255.0) as u8, (p * 255.0) as u8]),
        1 => color = Rgb([(q * 255.0) as u8, (value * 255.0) as u8, (p * 255.0) as u8]),
        2 => color = Rgb([(p * 255.0) as u8, (value * 255.0) as u8, (t * 255.0) as u8]),
        3 => color = Rgb([(p * 255.0) as u8, (value * 255.0) as u8, (q * 255.0) as u8]),
        4 => color = Rgb([(t * 255.0) as u8, (value * 255.0) as u8, (q * 255.0) as u8]),
        5 => color = Rgb([(value * 255.0) as u8, (p * 255.0) as u8, (q * 255.0) as u8]),
        _ => {}
    }
    color
}

