use crate::{
    math::{
        vec2::Vec2,
    },
    game::Game,
};

pub struct Blueprint {
    pub translation: Option<Vec2>,
    pub rotation: Option<f32>,
    pub scale: Option<Vec2>,
    pub using: Vec<Box<dyn Fn(&mut Game, usize) -> ()>>,
}
