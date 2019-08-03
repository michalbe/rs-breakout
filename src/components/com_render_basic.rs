use gl::types::*;
use crate::{
    components::com_render::*,
    materials::mat_common::{
        Material,
        Shape,
    },
    game::Game,
    components::Components,
};

pub struct RenderBasic {
    pub material: Material,
    pub vao: GLuint,
    pub count: i32,
    pub color: [f32; 4]
}

impl RenderBasic {
    pub fn empty() -> RenderBasic {
        RenderBasic {
            material: Material::new(),
            vao: 0,
            count: 0,
            color: [0.0, 0.0, 0.0, 0.0]
        }
    }

    pub fn new(material_type: &Material, shape: &Shape, color: [f32; 4]) -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            // vaos & materials
            game.world[entity] |= 1 << Components::Render as i32;
        }
    }
}

fn buffer(shape: Shape) -> GLuint {
    1
}
