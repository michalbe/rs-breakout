
use crate::game::Game;
use crate::math::vec2::Vec2;
use crate::components::Has;

#[derive(Clone, Copy)]
pub struct Collision {
    pub entity: usize,
    pub hit: Vec2,
}

#[derive(Clone, Copy)]
pub struct Collide {
    pub entity: usize,
    pub size: Vec2,
    pub min: Vec2,
    pub max: Vec2,
    pub center: Vec2,
    pub collision: Option<Collision>,
}

impl Collide {
    pub fn empty() -> Collide {
        Collide {
            entity: 0,
            size: Vec2::empty(),
            min: Vec2::empty(),
            max: Vec2::empty(),
            center: Vec2::empty(),
            collision: None,
        }
    }

    pub fn new(size: Option<Vec2>) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::Collide as u32;

            game.collide[entity] = Some(Collide {
                entity,
                size: match size {
                    Some(size_value) => { size_value },
                    None => { Vec2::empty() }
                },
                min: Vec2::empty(),
                max: Vec2::empty(),
                center: Vec2::empty(),
                collision: None,
            });
        }
    }
}
