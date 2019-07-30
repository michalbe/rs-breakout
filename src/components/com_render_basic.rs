use gl::types::*;
use crate::{
    components::com_render::*,
    materials::mat_common::Material,
};

pub struct RenderBasic<'a> {
    kind: RenderKind,
    material: Material<'a>,
    vao: GLuint,
    count: i32,
    color: [f32; 4]
}

