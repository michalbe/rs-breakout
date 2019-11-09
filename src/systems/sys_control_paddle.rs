use crate::{
    components::{
        Has,
    },
    game::{
        MAX_ENTITIES,
        Game,
    }
};

const QUERY: u32 = Has::Move as u32 | Has::ControlPaddle as u32;

pub fn sys_control_paddle(game: &mut Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i);
        }
    }

}

fn update(game: &mut Game, entity: usize) {
    if let (
        Some(mut move_component),
    ) = (
        game.move_component[entity],
    ) {

        move_component.direction.x = 0.0;

        if let Some(_) = game.input_state[0] {
            move_component.direction.x -= 1.0;
        }

        if let Some(_) = game.input_state[1] {
            move_component.direction.x += 1.0;
        }

        game.move_component[entity] = Some(move_component);
    }
}
