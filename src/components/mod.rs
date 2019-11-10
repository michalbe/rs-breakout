pub mod com_transform2d;
pub mod com_draw2d;
pub mod com_move;
pub mod com_controll_ball;
pub mod com_collide;

pub enum Component {
    Transform2d = 1,
    Draw2d,
    Move,
    ControlBall,
    Collide,
}

pub enum Has {
    Transform2d = 1 << Component::Transform2d as u32,
    Draw2d = 1 << Component::Draw2d as u32,
    Move = 1 << Component::Move as u32,
    ControlBall = 1 << Component::ControlBall as u32,
    Collide = 1 << Component::Collide as u32,
}
