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
    if let (
        Some(mut transform),
    ) = (
        game.transform[entity],
    ) {
        if transform.dirty {
            for child in transform.children.iter() {
                if let Some(child_entity_id) = child {
                    if let Some(mut child_transform) = game.transform[*child_entity_id] {
                        child_transform.dirty = true;
                        game.transform[*child_entity_id] = Some(child_transform);
                    }
                }
            }

            transform.dirty = false;

            let translated = Mat2d::from_translation(transform.translation);
            let translated_and_rotated = Mat2d::rotate(translated, transform.rotation);
            let translated_rotated_and_scaled = Mat2d::scale(translated_and_rotated, transform.scale);

            if let Some(parent_id) = transform.parent {
                if entity == 1 {
                    println!("before {}, {}", transform.world.m11, transform.world.m12);
                }
                let parent_transform = game.transform[parent_id].unwrap();
                // println!("parent: {}", parent_id);
                transform.world = Mat2d::multiply(parent_transform.world, translated_rotated_and_scaled);
            } else {
                transform.world = translated_rotated_and_scaled;
            }


            transform.self_mat = Mat2d::invert(transform.world);

            if entity == 1 {
               println!("before {}, {}", transform.world.m11, transform.world.m12);
            }

            // TODO: This is terrible, fixme!
            game.transform[entity] = Some(transform);
        }
    };
}
