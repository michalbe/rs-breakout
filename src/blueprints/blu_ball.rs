use crate::game::Game;
use crate::blueprints::blu_common::Blueprint;
use crate::math::vec2::Vec2;
use crate::components::com_draw2d::Draw2d;

pub fn get_ball(game: &mut Game, x: f32, y: f32) -> Blueprint {
    Blueprint {
        translation: Some(Vec2::new(x, y)),
        rotation: None,
        scale: None,
        using: vec![
            Box::new(Draw2d::new(Some(20), Some(20), Some([0, 255, 0, 255])))
        ],
    }
}

