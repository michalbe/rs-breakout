use crate::math::mat4::Mat4;

#[derive(Clone, Copy)]
pub struct Transform {
    world: Mat4,
    self_mat: Mat4,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            world: Mat4::new(),
            self_mat: Mat4::new(),
        }
    }
}
