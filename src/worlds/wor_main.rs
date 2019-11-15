
use crate::blueprints::blu_paddle::get_paddle;
use crate::blueprints::blu_block::get_block;
use crate::blueprints::blu_ball::get_ball;
use crate::game::Game;

pub fn world_main(game: &mut Game) {
    let column_count = 5;
    let row_count = 5;
    let block_width = 100;
    let block_height = 20;
    let margin = 10;
    let grid_width = block_width * column_count + margin * (column_count - 1);
    let starting_x = (game.window_width - grid_width)/2;
    let starting_y = 50;

    let paddle = get_paddle((game.window_width / 2) as f32, (game.window_height - 30) as f32);
    game.add(paddle);

    let ball = get_ball((game.window_width/2) as f32, (game.window_height/2) as f32);
    game.add(ball);

    for row in 0..row_count {
        let y = starting_y + row * (block_height + margin) + block_height / 2;
        for col in 0..column_count {
            let x = starting_x + col * (block_width + margin) + block_width / 2;
            let block = get_block(x as f32, y as f32, block_width, block_height);
            game.add(block);
        }
    }
}
