
use crate::blueprints::blu_common::Blueprint;
use crate::blueprints::blu_paddle::get_paddle;
use crate::blueprints::blu_block::get_block;
use crate::blueprints::blu_ball::get_ball;
use crate::game::Game;

pub fn world_children_test(game: &mut Game) {
    let mut camera = Blueprint {
        translation: None,
        rotation: None,
        scale: None,
        using: vec![],
        children: None,
    };

    let mut ball = get_ball((game.window_width/2) as f32, (game.window_height/2) as f32);
    let mut ball2 = get_ball((game.window_width/2 + 30) as f32, (game.window_height/2 + 30) as f32);

    let camera_children = Some(vec![ball, ball2]);

    camera.children = camera_children;

    game.add(camera);
}
