mod game;
mod math;
mod materials;
mod components;
mod blueprints;

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
