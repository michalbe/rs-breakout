use crate::math::{
    mat4::Mat4,
    vec3::Vec3,
};

#[derive(Clone, Copy)]
pub struct Transform {
    // Matrix relative to the world
    world: Mat4,
    // World to self matrix
    self_mat: Mat4,

    // translation relative to the parent
    translation: Vec3,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            world: Mat4::new(),
            self_mat: Mat4::new(),
            translation: Vec3::new(),
        }
    }
}
