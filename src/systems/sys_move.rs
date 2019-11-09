
use crate::components::com_transform2d::Transform2d;
use crate::math::mat2d::Mat2d;
use crate::{
    components::{
        Has,
    },
    game::{
        MAX_ENTITIES,
        Game,
    }
};

const QUERY: u32 = Has::Transform2d as u32 | Has::Move as u32;

pub fn sys_move(game: &mut Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i, delta);
        }
    }

}

fn update(game: &mut Game, entity: usize, delta: f32) {
    if let (Some(mut transform), Some(move_component)) = (game.transform[entity], game.move_component[entity]) {
        if move_component.direction.x != 0.0 || move_component.direction.y != 0.0 {
            transform.translation.x += move_component.direction.x * move_component.speed * delta;
            transform.translation.y += move_component.direction.y * move_component.speed * delta;

            transform.dirty = true;

            // TODO: This is terrible, fixme!
            game.transform[entity] = Some(transform);
        }
    };
}
