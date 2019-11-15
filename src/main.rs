mod game;
mod math;
mod components;
mod blueprints;
mod systems;
mod worlds;

use crate::worlds::wor_children_test::world_children_test;
// use crate::worlds::wor_main::world_main;
use crate::game::Game;

fn main() {
    let mut game = Game::new(640, 480);

    // world_main(&mut game);
    world_children_test(&mut game);
    game.start();
}
