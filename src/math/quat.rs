#[derive(Clone, Copy)]
pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quat {
    pub fn new() -> Quat {
        Quat {
            x: 0,
            y: 0,
            z: 0,
            w: 1,
        }
    }
}
