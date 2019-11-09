use crate::{
    math::{
        mat2d::Mat2d,
        vec2::Vec2,
    },
    game::Game,
    components::Has,
};

#[derive(Clone, Copy)]
pub struct Move {
    pub direction: Vec2,
    pub speed: f32,
}

impl Move {
    pub fn empty() -> Move {
        Move {
            direction: Vec2::empty(),
            speed: 1.0,
        }
    }

    pub fn new(direction: Option<Vec2>, speed: Option<f32>) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::Move as u32;

            game.move_component[entity] = Some(Move {
                direction: match direction {
                    Some(direction_value) => { direction_value },
                    None => { Vec2::empty() }
                },
                speed: match speed {
                    Some(speed_value) => { speed_value },
                    None => { 0.0 }
                },
            });
        }
    }
}
