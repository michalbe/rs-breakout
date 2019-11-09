mod game;
mod math;
mod components;
mod blueprints;
mod systems;

use crate::math::vec2::Vec2;
use crate::components::com_draw2d::Draw2d;
use crate::{
    game::*,
    blueprints::blu_common::*,
};

fn main() {
    let mut game = Game::new(640, 480);
    let translation = Vec2::new(100.0, 200.0);
    let rotation = 0.0;
    let scale = Vec2::new(2.0, 2.0);

    let mut blueprint_without_mixins = Blueprint {
        translation: Some(Vec2::new(200.0, 300.0)),
        rotation: Some(rotation),
        scale: Some(scale),
        using: vec![
            Box::new(Draw2d::new(Some(100), Some(20), Some([0, 255, 0, 255])))
        ],
    };

    let mut blueprint_with_mixins = Blueprint {
        translation: Some(translation),
        rotation: Some(rotation),
        scale: Some(scale),
        using: vec![
            Box::new(Draw2d::new(Some(50), Some(50), Some([255, 0, 255, 255])))
        ],
    };

    let entity_1 = game.add(&mut blueprint_without_mixins);
    let entity_2 = game.add(&mut blueprint_with_mixins);

    game.start();
}
