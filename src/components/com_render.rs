use crate::{
    components::com_render_basic::RenderBasic,
};

#[derive(Clone, Copy)]
pub enum RenderKind {
   Basic,
   Shaded,
}

pub enum Render {
    RenderBasic(RenderBasic)
}
