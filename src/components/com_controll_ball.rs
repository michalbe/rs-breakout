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

    pub fn new(angle: Option<f32>) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::ControlBall as u32;

            game.control_ball[entity] = Some(ControlBall {
                direction: match angle {
                    Some(angle) => {
                        let x = angle.cos();
                        let y = angle.sin();
                        Vec2::new(x, y)
                     },
                    None => {
                        let default_angle: f32 = std::f32::consts::PI * 0.33;
                        let x = default_angle.cos();
                        let y = default_angle.sin();
                        Vec2::new(x, y)
                    }
                },
            });
        }
    }
}
