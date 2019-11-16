use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::components::Has;
use crate::game::MAX_ENTITIES;
use crate::game::Game;

const QUERY: u32 = Has::Transform2d as u32 | Has::Draw2d as u32;

pub fn sys_draw2d(game: &mut Game, _delta: f32) {
    game.canvas.set_draw_color(Color::RGB(game.clear_color[0], game.clear_color[1], game.clear_color[2]));
    game.canvas.clear();
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i);
        }
    }

}

fn update(game: &mut Game, entity: usize) {
    if let (Some(transform), Some(draw2d)) = (game.transform[entity], game.draw2d[entity]) {
        let width = (draw2d.width as f32 * transform.scale.x) as i32;
        let height = (draw2d.height as f32 * transform.scale.y) as i32;

        game.canvas.set_draw_color(Color::RGBA(draw2d.color[0], draw2d.color[1], draw2d.color[2], draw2d.color[3]));
        game.canvas.fill_rect(Rect::new(
            transform.world.m11 as i32 - (width / 2) as i32,
            transform.world.m12 as i32 - (height / 2) as i32,
            width as u32,
            height as u32)
        ).unwrap();
    };
}
