#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn empty() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, ) -> Vec2 {
        Vec2 { x, y }
    }
}
