#[derive(Clone, Copy)]
pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quat {
    pub fn empty() -> Quat {
        Quat::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quat {
        Quat { x, y, z, w,}
    }
}
