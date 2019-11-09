
use crate::components::com_transform2d::Transform2d;
use crate::{
    components::{
        Has,
    },
    game::{
        MAX_ENTITIES,
        Game,
    }
};

const QUERY: u32 = Has::Transform2d as u32;

pub fn sys_transform2d(game: &Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i);
        }
    }

}

fn update(game: &Game, entity: usize) {
    match game.transform[entity] {
        Some(mut transform) => {
            if transform.dirty {
                transform.dirty = false;
                println!("transform system works!");
            }
        }
        None => {}
    };

}
