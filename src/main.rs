mod game;
mod components;
mod math;

use crate::game::*;

fn main() {
    let mut game = Game::new();
    let entity_1 = game.create_entity(2);
    let entity_2 = game.create_entity(2);

    println!("{}, {}", entity_1, entity_2);
}
