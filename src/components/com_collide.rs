
use crate::math::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Collision {
    pub entity: usize,
    pub hit: Vec2,
}

#[derive(Clone, Copy)]
pub struct Collide {
    pub entity: usize,
    pub size: Vec2,
    pub min: Vec2,
    pub max: Vec2,
    pub center: Vec2,
    pub collision: Collision,
}


