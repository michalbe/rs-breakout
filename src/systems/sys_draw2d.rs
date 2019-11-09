use crate::components::com_transform2d::Transform2d;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use crate::{
    components::{
        Has,
    },
    game::{
        MAX_ENTITIES,
        Game,
    }
};

const QUERY: u32 = Has::Transform2d as u32 | Has::Draw2d as u32;

pub fn sys_draw2d(game: &mut Game, delta: f32) {
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
        game.canvas.set_draw_color(Color::RGB(draw2d.color[0], draw2d.color[1], draw2d.color[2]));
        game.canvas.fill_rect(Rect::new(
            transform.translation.x as i32 - (draw2d.width / 2) as i32,
            transform.translation.y as i32 - (draw2d.height / 2) as i32,
            draw2d.width,
            draw2d.height)
        ).unwrap();
    };
}
