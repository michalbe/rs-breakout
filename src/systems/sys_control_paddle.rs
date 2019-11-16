use sdl2::keyboard::Scancode;
use crate::components::Has;
use crate::game::MAX_ENTITIES;
use crate::game::Game;

const QUERY: u32 = Has::Move as u32 | Has::ControlPaddle as u32;

pub fn sys_control_paddle(game: &mut Game, _delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i);
        }
    }

}

fn update(game: &mut Game, entity: usize) {
    let move_component = game.move_component[entity].as_mut().unwrap();

    move_component.direction.x = 0.0;

    if let Some(_) = game.input_state[Scancode::Left as usize] {
        move_component.direction.x -= 1.0;
    }

    if let Some(_) = game.input_state[Scancode::Right as usize] {
        move_component.direction.x += 1.0;
    }
}
