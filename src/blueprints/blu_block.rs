use crate::blueprints::blu_common::Blueprint;
use crate::math::vec2::Vec2;
use crate::components::com_draw2d::Draw2d;
use crate::components::com_collide::Collide;
use crate::components::com_control_block::ControlBlock;

pub fn get_block(x: f32, y: f32, width: u32, height: u32) -> Blueprint {
    Blueprint {
        translation: Some(Vec2::new(x, y)),
        rotation: None,
        scale: None,
        using: vec![
            Box::new(Draw2d::new(Some(width), Some(height), Some([255, 255, 0, 255]))),
            Box::new(Collide::new(Some(Vec2::new(width as f32, height as f32)))),
            Box::new(ControlBlock::new()),
        ],
        children: None,
    }
}

