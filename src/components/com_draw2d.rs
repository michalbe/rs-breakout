use crate::{
    math::{
        mat2d::Mat2d,
        vec2::Vec2,
    },
    game::Game,
    components::Has,
};

#[derive(Clone, Copy)]
pub struct Draw2d {
    pub width: f32,
    pub height: f32,
    // TODO: Will anything smaller fit here as well? like u8?
    pub color: [u32; 4]
}

impl Draw2d {
    pub fn empty() -> Draw2d {
        Draw2d {
            width: 10.0,
            height: 10.0,
            color: [255, 0, 255, 255]
        }
    }

    pub fn new(width: Option<f32>, height: Option<f32>, color: Option<[u32; 4]>) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::Draw2d as u32;

            game.draw2d[entity] = Some(Draw2d {
                width: match width {
                    Some(width_value) => { width_value },
                    None => { 10.0 }
                },
                height: match height {
                    Some(height_value) => { height_value },
                    None => { 10.0 }
                },
                color: match color {
                    Some(color_value) => { color_value },
                    None => { [255, 0, 255, 255] }
                }
            });
        }
    }
}
