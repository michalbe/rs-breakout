mod game;
mod math;
mod components;
mod blueprints;
mod systems;

use crate::systems::sys_transform2d::sys_transform2d;
use crate::{
    game::*,
    blueprints::blu_common::*,
};

fn main() {
    let mut game = Game::new();

    let mut blueprint = Blueprint {
        translation: None,
        rotation: None,
        scale: None,
        using: vec![],
    };

    let entity_1 = game.add(&mut blueprint);
    let entity_2 = game.add(&mut blueprint);

    println!("{}, {}", entity_1, entity_2);
}
