use crate::components::Has;
use crate::game::MAX_ENTITIES;
use crate::game::Game;

const QUERY: u32 = Has::Transform2d as u32 | Has::Move as u32 | Has::ControlBall as u32;

pub fn sys_control_ball(game: &mut Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i, delta);
        }
    }

}

fn update(game: &mut Game, entity: usize, _delta: f32) {
    if let (
        Some(mut transform),
        Some(mut control),
        Some(mut move_component),
    ) = (
        game.transform[entity],
        game.control_ball[entity],
        game.move_component[entity],
    ) {
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

       if let Some(collide) = game.collide[entity] {
            if let Some(collision) = collide.collision {
                let mut camera_shake = game.shake[game.camera].unwrap();
                camera_shake.duration = 0.2;
                game.shake[game.camera] = Some(camera_shake);

                if collision.hit.x != 0.0 {
                    transform.translation.x += collision.hit.x;
                    control.direction.x *= -1.0;
                }
                if collision.hit.y != 0.0 {
                    transform.translation.y += collision.hit.y;
                    control.direction.y *= -1.0;
                }
            }
       }

       move_component.direction.x = control.direction.x;
       move_component.direction.y = control.direction.y;

        let ball_fade = crate::blueprints::blu_common::Blueprint {
            translation: Some(crate::math::vec2::Vec2::new(transform.translation.x, transform.translation.y)),
            rotation: None,
            scale: None,
            using: vec![
                Box::new(crate::components::com_draw2d::Draw2d::new(Some(20), Some(20), Some([0, 255, 0, 255]))),
                Box::new(crate::components::com_fade::Fade::new(Some(0.05))),
            ],
            children: None,
        };

        game.add(ball_fade);


       // TODO: This is terrible, fixme!
       game.move_component[entity] = Some(move_component);
       game.transform[entity] = Some(transform);
       game.control_ball[entity] = Some(control);
    };
}
