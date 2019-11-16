use crate::game::MAX_ENTITIES;
use crate::game::Game;
use crate::components::Has;

const QUERY: u32 = Has::Transform2d as u32 | Has::Move as u32;

pub fn sys_move(game: &mut Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i, delta);
        }
    }

}

fn update(game: &mut Game, entity: usize, delta: f32) {
    let mut transform = game.transform[entity].as_mut().unwrap();
    let move_component = game.move_component[entity].as_mut().unwrap();

    if move_component.direction.x != 0.0 || move_component.direction.y != 0.0 {
        transform.translation.x += move_component.direction.x * move_component.speed * delta;
        transform.translation.y += move_component.direction.y * move_component.speed * delta;

        transform.dirty = true;
    }
}
