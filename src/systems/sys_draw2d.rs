use crate::components::com_transform2d::Transform2d;
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

pub fn sys_draw2d(game: &Game, delta: f32) {
    // TODO: Clear screen here
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i);
        }
    }

}

fn update(game: &Game, entity: usize) {
    // TODO: Can this be handled smarter? I don't think
    // I like creating tuples like that...
    if let (
        Some(transform), Some(draw2d)) = (game.transform[entity], game.draw2d[entity]
    ) {
        println!("draw2d x:{}, y:{}", transform.translation.x - draw2d.width / 2.0, transform.translation.y - draw2d.height / 2.0);
    };
}
