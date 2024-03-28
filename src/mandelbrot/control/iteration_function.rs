pub fn mandelbrot(x: f64, y: f64, max_iterations: u32) -> u32 {
    const THRESHOLD: f64 = 4.0;

    let mut zx = 0.0;
    let mut zy = 0.0;
    let mut i = 0;

    while i < max_iterations {
        let zx_squared = zx * zx;
        let zy_squared = zy * zy;

        if zx_squared + zy_squared > THRESHOLD {
            break;
        }

        zy = 2.0 * zx * zy + y;
        zx = zx_squared - zy_squared + x;

        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_in_mandelbrot_set() {
        let x = 0.0;
        let y = 0.0;
        let max_iterations = 20;

        let result = mandelbrot(x, y, max_iterations);

        assert_eq!(result, max_iterations);
    }
}
