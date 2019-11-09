use crate::{
    math::{
        mat2d::Mat2d,
        vec2::Vec2,
    },
    game::Game,
    components::Has,
};

#[derive(Clone, Copy)]
pub struct ControlBall {
    pub direction: Vec2,
}

impl ControlBall {
    pub fn empty() -> ControlBall {
        ControlBall {
            direction: Vec2::empty(),
        }
    }

    pub fn new(direction: Option<Vec2>) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::ControlBall as u32;

            game.control_ball[entity] = Some(ControlBall {
                direction: match direction {
                    Some(direction_value) => { direction_value },
                    None => { Vec2::empty() }
                },
            });
        }
    }
}
