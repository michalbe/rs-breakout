mod game;
mod math;
mod components;
mod blueprints;

use crate::{
    game::*,
    blueprints::blu_common::*,
};

fn main() {
    let mut game = Game::new();

    let blueprint = Blueprint {
        translation: None,
        rotation: None,
        scale: None,
        // using: vec![],
    };

    let entity_1 = game.add(&blueprint);
    let entity_2 = game.add(&blueprint);

    println!("{}, {}", entity_1, entity_2);
}
