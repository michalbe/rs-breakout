use crate::game::Game;
use crate::components::Has;

#[derive(Clone, Copy)]
pub struct Fade {
    pub step: f32,
}

impl Fade {
    pub fn new(step: Option<f32>) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::Fade as u32;

            game.fade[entity] = Some(Fade {
                step: match step {
                    Some(value) => { value },
                    None => { 0.0 }
                },
            });
        }
    }
}
