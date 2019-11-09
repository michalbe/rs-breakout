pub mod com_transform2d;
pub mod com_draw2d;
pub mod com_move;
pub mod com_controll_ball;
pub mod com_collide;
pub mod com_control_block;
pub mod com_control_paddle;

pub enum Component {
    Transform2d = 1,
    Draw2d,
    Move,
    ControlBall,
    Collide,
    ControlBlock,
    ControlPaddle,
}

pub enum Has {
    Transform2d = 1 << Component::Transform2d as u32,
    Draw2d = 1 << Component::Draw2d as u32,
    Move = 1 << Component::Move as u32,
    ControlBall = 1 << Component::ControlBall as u32,
    Collide = 1 << Component::Collide as u32,
    ControlBlock = 1 << Component::ControlBlock as u32,
    ControlPaddle = 1 << Component::ControlPaddle as u32,
}
