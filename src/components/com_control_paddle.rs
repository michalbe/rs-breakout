use crate::{
    game::Game,
    components::Has,
};

#[derive(Clone, Copy)]
pub struct ControlPaddle { }

impl ControlPaddle {
    pub fn new() -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::ControlPaddle as u32;
        }
    }
}
