
use crate::blueprints::blu_ball::get_ball;
use crate::blueprints::blu_common::Blueprint;
use crate::game::Game;
use crate::math::vec2::Vec2;
use crate::components::com_draw2d::Draw2d;

pub fn world_main(game: &mut Game) {
    let mut ball = get_ball(game, (game.window_width/2) as f32, (game.window_height/2) as f32);
    game.add(&mut ball);
}
