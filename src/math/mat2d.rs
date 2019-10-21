#[derive(Clone, Copy)]
pub struct Mat2d {
    pub m00: f64,
    pub m01: f64,
    pub m02: f64,
    pub m03: f64,
    pub m10: f64,
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
}

impl Mat2d {
    pub fn empty() -> Mat2d {
        Mat2d {
            m00: 1.0,
            m01: 0.0,
            m02: 0.0,
            m03: 0.0,
            m10: 1.0,
            m11: 0.0,
            m12: 0.0,
            m13: 0.0,
        }
    }
}
