pub mod com_transform2d;
pub mod com_draw2d;
pub mod com_move;

pub enum Component {
    Transform2d = 1,
    Draw2d,
    Move
}

pub enum Has {
    Transform2d = 1 << Component::Transform2d as u32,
    Draw2d = 1 << Component::Draw2d as u32,
    Move = 1 << Component::Move as u32,
}
