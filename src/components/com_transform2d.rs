use crate::{
    math::{
        mat2d::Mat2d,
        vec2::Vec2,
    },
    game::Game,
    components::Has,
};

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

    // TODO: in original Goodluck Trannsform::parent is a Transform
    // instance. Will this work as well?
    // pub parent: Option<usize>,
    // TODO: This needs a little bit more thinking.
    // pub children: Vec<Transform>,
    pub dirty: bool,
}

impl Transform2d {
    pub fn empty() -> Transform2d {
        Transform2d {
            world: Mat2d::empty(),
            self_mat: Mat2d::empty(),
            translation: Vec2::empty(),
            rotation: 0.0,
            scale: Vec2::empty(),

            entity_id: 0,
            // parent: None,
            // children: Default::default(),
            dirty: true,
        }
    }

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
                // parent: None,
                // children: Default::default(),
                dirty: true,
            });
        }
    }
}
