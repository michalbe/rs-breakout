use crate::game::MAX_CHILDREN;
use crate::math::mat2d::Mat2d;
use crate::math::vec2::Vec2;
use crate::game::Game;
use crate::components::Has;

#[derive(Clone, Copy)]
pub struct Transform2d {
    // Matrix relative to the world
    pub world: Mat2d,
    // World to self matrix
    pub self_mat: Mat2d,

    // local translation relative to the parent
    pub translation: Vec2,
    // local rotation relative to the parent
    pub rotation: f32,
    // local scale relative to the parent
    pub scale: Vec2,

    pub entity_id: usize,
    pub parent: Option<usize>,
    pub children: [Option<usize>; MAX_CHILDREN],
    pub dirty: bool,
}

impl Transform2d {
    pub fn new(translation: Option<Vec2>, rotation: Option<f32>, scale: Option<Vec2>) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::Transform2d as u32;

            game.transform[entity] = Some(Transform2d {
                world: Mat2d::empty(),
                self_mat: Mat2d::empty(),
                translation: match translation {
                    Some(translation_value) => { translation_value },
                    None => { Vec2::empty() }
                },
                rotation: match rotation {
                    Some(rotation_value) => { rotation_value },
                    None => { 0.0 }
                },
                scale: match scale {
                    Some(scale_value) => { scale_value },
                    None => { Vec2::new(1.0, 1.0) }
                },
                entity_id: entity,
                parent: None,
                children: [None; MAX_CHILDREN],
                dirty: true,
            });
        }
    }
}
