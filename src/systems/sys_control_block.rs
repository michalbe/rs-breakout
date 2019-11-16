use crate::blueprints::blu_ball::get_ball;
use crate::components::Has;
use crate::game::MAX_ENTITIES;
use crate::game::Game;

const QUERY: u32 = Has::ControlBlock as u32 | Has::Collide as u32 | Has::Fade as u32;

pub fn sys_control_block(game: &mut Game, _delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i);
        }
    }

}

fn update(game: &mut Game, entity: usize) {
    if let (
        Some(collide),
        Some(mut fade),
    ) = (
        game.collide[entity],
        game.fade[entity],
    ) {

        if let Some(_) = collide.collision {
            fade.step = 0.02;
            game.fade[entity] = Some(fade);
            // game.world[entity] &= Has::Collide as u32;
            // if rand::random::<f32>() > 0.98 {
            //     game.add(get_ball((game.window_width/2) as f32, (game.window_height/2) as f32));
            // }
        }
    }
}
