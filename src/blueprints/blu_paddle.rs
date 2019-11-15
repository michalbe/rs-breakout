use crate::blueprints::blu_common::Blueprint;
use crate::math::vec2::Vec2;
use crate::components::com_draw2d::Draw2d;
use crate::components::com_collide::Collide;
use crate::components::com_control_paddle::ControlPaddle;
use crate::components::com_move::Move;

pub fn get_paddle(x: f32, y: f32) -> Blueprint {
    Blueprint {
        translation: Some(Vec2::new(x, y)),
        rotation: None,
        scale: None,
        using: vec![
            Box::new(Draw2d::new(Some(100), Some(20), Some([255, 0, 0, 255]))),
            Box::new(Collide::new(Some(Vec2::new(100.0, 20.0)))),
            Box::new(Move::new(None, Some(90.0))),
            Box::new(ControlPaddle::new()),
        ],
        children: None,
    }
}

