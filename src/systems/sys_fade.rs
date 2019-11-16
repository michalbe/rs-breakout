use crate::game::MAX_ENTITIES;
use crate::game::Game;
use crate::components::Has;

const QUERY: u32 = Has::Transform2d as u32 | Has::Fade as u32 | Has::Draw2d as u32;

pub fn sys_fade(game: &mut Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i, delta);
        }
    }

}

fn update(game: &mut Game, entity: usize, delta: f32) {
    if let (
        Some(mut transform), Some(fade), Some(mut draw)
    ) = (
        game.transform[entity], game.fade[entity], game.draw2d[entity]
    ) {

        if draw.color[3] > 0 {
            let factor = 1.0 / fade.step;
            draw.color[3] = (draw.color[3] as f32 * factor) as u8;
            transform.scale.x = factor;
            transform.scale.y = factor;
            transform.dirty = true;

            if draw.color[3] <= 0 {
                game.destroy(entity);
            }

            game.transform[entity] = Some(transform);
            game.draw2d[entity] = Some(draw);
        }
    };
}
