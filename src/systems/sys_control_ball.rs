
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

const QUERY: u32 = Has::Transform2d as u32 | Has::Move as u32 | Has::ControlBall as u32;

pub fn sys_control_ball(game: &mut Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i, delta);
        }
    }

}

fn update(game: &mut Game, entity: usize, delta: f32) {
    if let (Some(mut transform), Some(mut control), Some(mut move_component)) = (game.transform[entity], game.control_ball[entity], game.move_component[entity]) {
       if transform.translation.x < 0.0 {
            transform.translation.x = 0.0;
            control.direction.x *= -1.0;
       }

       if transform.translation.x > game.window_width as f32 {
            transform.translation.x = game.window_width as f32;
            control.direction.x *= -1.0;
       }

       if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            control.direction.y *= -1.0;
       }

        if transform.translation.y > game.window_height as f32 {
            transform.translation.y = game.window_height as f32;
            control.direction.y *= -1.0;
       }

       move_component.direction.x = control.direction.x;
       move_component.direction.y = control.direction.y;

       // TODO: This is terrible, fixme!
       game.move_component[entity] = Some(move_component);
       game.transform[entity] = Some(transform);
       game.control_ball[entity] = Some(control);
    };
}
