use crate::math::vec2::Vec2;
use crate::blueprints::blu_common::Blueprint;
use crate::components::com_draw2d::Draw2d;
use crate::components::com_move::Move;
use crate::components::com_fade::Fade;

pub fn get_explosion(x: f32, y: f32) -> Blueprint {
    let explosions = 32;
    let mut children: Vec<Blueprint> = vec![];
    let step = (std::f32::consts::PI * 2.0) / explosions as f32;
    let fade_step = 0.05;

    for i in 0..explosions {
        children.push(Blueprint {
            translation: None,
            rotation: None,
            scale: None,
            using: vec![
                Box::new(Draw2d::new(Some(20), Some(20), Some([255, 255, 255, 255]))),
                Box::new(Move::new(Some(Vec2::new((step * i as f32).sin(), (step * i as f32).cos())), Some(800.0))),
                Box::new(Fade::new(Some(fade_step))),
            ],
            children: None,
        });
    }

    Blueprint {
        translation: Some(Vec2::new(x, y)),
        rotation: None,
        scale: None,
        using: vec![],
        children: Some(children),
    }
}

