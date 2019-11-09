#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn empty() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    pub fn new(x: f32, y: f32, ) -> Vec2 {
        Vec2 { x, y }
    }
}
