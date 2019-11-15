use crate::game::Game;
use crate::components::Has;

#[derive(Clone, Copy)]
pub struct Shake {
    pub duration: f32,
    pub strength: f32,
}

impl Shake {
    pub fn new(duration: Option<f32>, strength: Option<f32>) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::Shake as u32;

            game.shake[entity] = Some(Shake {
                duration: match duration {
                    Some(value) => { value },
                    None => { 1.0 }
                },
                strength: match strength {
                    Some(value) => { value },
                    None => { 0.0 }
                },
            });
        }
    }
}
