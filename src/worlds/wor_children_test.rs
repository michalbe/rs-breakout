
use crate::blueprints::blu_common::Blueprint;
use crate::blueprints::blu_paddle::get_paddle;
use crate::blueprints::blu_block::get_block;
use crate::blueprints::blu_ball::get_ball;
use crate::game::Game;
use crate::components::com_draw2d::Draw2d;

pub fn world_children_test(game: &mut Game) {
    let mut camera = Blueprint {
        translation: None,
        rotation: None,
        scale: None,
        using: vec![
            Box::new(crate::components::com_move::Move::new(Some(crate::math::vec2::Vec2::new(1.0, 1.0)), Some(30.0))),
            Box::new(crate::components::com_controll_ball::ControlBall::new(None)),
        ],
        children: None,
    };

    let ball = get_ball((game.window_width/2) as f32, (game.window_height/2) as f32);
    let ball2 = get_ball((game.window_width/2 + 30) as f32, (game.window_height/2 + 30) as f32);

    let unmovable = Blueprint {
        translation: None,
        rotation: None,
        scale: None,
        using: vec![
            // Box::new(crate::components::com_move::Move::new(Some(crate::math::vec2::Vec2::new(1.0, 0.0)), Some(30.0))),
            Box::new(Draw2d::new(Some(50), Some(50), Some([123, 055, 031, 255]))),
        ],
        children: None,
    };


    let camera_children = Some(vec![unmovable, ball, ball2]);

    camera.children = camera_children;

    game.add(camera);
}
