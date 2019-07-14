use crate::math::{
    mat4::Mat4,
    vec3::Vec3,
    quat::Quat,
};

#[derive(Clone, Copy)]
pub struct Transform {
    // Matrix relative to the world
    pub world: Mat4,
    // World to self matrix
    pub self_mat: Mat4,

    // local translation relative to the parent
    pub translation: Vec3,
    // local rotation relative to the parent
    pub rotation: Quat,
    // local scale relative to the parent
    pub scale: Vec3,

    pub entity_id: usize,
    pub parent: Option<usize>,
    // pub children: Vec<Transform>,
    pub dirty: bool,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            world: Mat4::empty(),
            self_mat: Mat4::empty(),
            translation: Vec3::empty(),
            rotation: Quat::empty(),
            scale: Vec3::empty(),

            entity_id: 0,
            parent: None,
            // children: Default::default(),
            dirty: true,
        }
    }
}
