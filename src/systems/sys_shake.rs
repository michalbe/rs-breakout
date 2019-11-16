use crate::game::MAX_ENTITIES;
use crate::game::Game;
use crate::components::Has;

const QUERY: u32 = Has::Transform2d as u32 | Has::Shake as u32;

pub fn sys_shake(game: &mut Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i, delta);
        }
    }

}

fn update(game: &mut Game, entity: usize, delta: f32) {
    if let (Some(mut transform), Some(mut shake)) = (game.transform[entity], game.shake[entity]) {

        if shake.duration > 0.0 {
            shake.duration -= delta;
            transform.translation.x = shake.strength - rand::random::<f32>() * (shake.strength * 2.0);
            transform.translation.y = shake.strength - rand::random::<f32>() * (shake.strength * 2.0);

            game.clear_color[0] = (rand::random::<f32>() * 255.0) as u8;
            game.clear_color[1] = (rand::random::<f32>() * 255.0) as u8;
            game.clear_color[2] = (rand::random::<f32>() * 255.0) as u8;

            transform.dirty = true;

            if shake.duration <= 0.0 {
                shake.duration = 0.0;
                transform.translation.x = 0.0;
                transform.translation.y = 0.0;
                game.clear_color[0] = 0;
                game.clear_color[1] = 0;
                game.clear_color[2] = 0;

            }

            game.transform[entity] = Some(transform);
            game.shake[entity] = Some(shake);
        }
    };
}
