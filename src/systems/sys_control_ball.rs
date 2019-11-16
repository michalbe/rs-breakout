use crate::blueprints::blu_explosion::get_explosion;
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
    let transform = game.transform[entity].as_mut().unwrap();
    let move_component = game.move_component[entity].as_mut().unwrap();
    let control = game.control_ball[entity].as_mut().unwrap();

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

    let mut explosion = false;
    if let Some(collide) = game.collide[entity] {
        if let Some(collision) = collide.collision {
            if let Some(ref mut camera_shake) = game.shake[game.camera] {
                camera_shake.duration = 0.2;
            };

            if collision.hit.x != 0.0 {
                transform.translation.x += collision.hit.x;
                control.direction.x *= -1.0;
            }
            if collision.hit.y != 0.0 {
                transform.translation.y += collision.hit.y;
                control.direction.y *= -1.0;
            }

            explosion = true;
        }
    }

    move_component.direction.x = control.direction.x;
    move_component.direction.y = control.direction.y;

    let x = transform.translation.x.clone();
    let y = transform.translation.y.clone();

    let ball_fade = crate::blueprints::blu_common::Blueprint {
        translation: Some(crate::math::vec2::Vec2::new(x, y)),
        rotation: None,
        scale: None,
        using: vec![
            Box::new(crate::components::com_draw2d::Draw2d::new(Some(20), Some(20), Some([0, 255, 0, 255]))),
            Box::new(crate::components::com_fade::Fade::new(Some(0.05))),
        ],
        children: None,
    };

    game.add(ball_fade);

    // XXX: Why this needs to be here, and not where the `explosion` flag is being set?
    if explosion {
        game.add(get_explosion(x, y));
    }
}
