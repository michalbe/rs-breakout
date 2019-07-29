use gl::types::*;
use crate::{
    components::com_render::*,
    materials::mat_common::Material,
};

pub struct RenderBasic {
    kind: RenderKind,
    material: Material,
    vao: GLuint,
    count: i32,
    color: [f32; 4]
}

