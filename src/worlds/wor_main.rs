use crate::blueprints::blu_common::Blueprint;
use crate::blueprints::blu_paddle::get_paddle;
use crate::blueprints::blu_block::get_block;
use crate::blueprints::blu_ball::get_ball;
use crate::game::Game;

pub fn world_main(game: &mut Game) {
    let column_count = 10;
    let row_count = 10;
    let block_width = 50;
    let block_height = 15;
    let margin = 5;
    let grid_width = block_width * column_count + margin * (column_count - 1);
    let starting_x = (game.window_width - grid_width)/2;
    let starting_y = 50;

    let mut camera = Blueprint {
        translation: None,
        rotation: None,
        scale: None,
        using: vec![
            Box::new(crate::components::com_shake::Shake::new(Some(0.0), Some(20.0))),
        ],
        children: None,
    };

    let mut game_elements = vec![
        get_paddle((game.window_width / 2) as f32, (game.window_height - 30) as f32),
        get_ball((game.window_width/2) as f32, (game.window_height - 60) as f32),
    ];


    for row in 0..row_count {
        let y = starting_y + row * (block_height + margin) + block_height / 2;
        for col in 0..column_count {
            let x = starting_x + col * (block_width + margin) + block_width / 2;
            game_elements.push(get_block(x as f32, y as f32, block_width, block_height));
        }
    }

    camera.children = Some(game_elements);

    game.camera = game.add(camera);
}
