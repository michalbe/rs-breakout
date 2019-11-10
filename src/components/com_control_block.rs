use crate::{
    math::{
        mat2d::Mat2d,
        vec2::Vec2,
    },
    game::Game,
    components::Has,
};

#[derive(Clone, Copy)]
pub struct ControlBlock { }

impl ControlBlock {
    pub fn new() -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::ControlBall as u32;
        }
    }
}
