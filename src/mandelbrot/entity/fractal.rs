use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fractal {
    pub c0: f64,
    pub c0i: f64,
    pub c1: f64,
    pub c1i: f64,
    pub width: u32,
    pub height: u32,
    pub max_iterations: u32,
}
