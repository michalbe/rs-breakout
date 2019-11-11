use crate::math::mat2d::Mat2d;
use crate::game::MAX_ENTITIES;
use crate::game::Game;
use crate::components::Has;

const QUERY: u32 = Has::Transform2d as u32;

pub fn sys_transform2d(game: &mut Game, _delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i);
        }
    }

}

fn update(game: &mut Game, entity: usize) {
    match game.transform[entity] {
        Some(mut transform) => {
            if transform.dirty {
                transform.dirty = false;
                let translated = Mat2d::from_translation(transform.translation);
                let translated_and_rotated = Mat2d::rotate(translated, transform.rotation);
                let translated_rotated_and_scaled = Mat2d::scale(translated_and_rotated, transform.scale);

                transform.world = translated_rotated_and_scaled;
                transform.self_mat = Mat2d::invert(translated_rotated_and_scaled);

                // TODO: This is terrible, fixme!
                game.transform[entity] = Some(transform);
            }
        }
        None => {}
    };

}
