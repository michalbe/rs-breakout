pub mod com_transform2d;

pub enum Component {
    Transform = 1,
    Render,
}

pub enum Has {
    Transform = 1 << Component::Transform as u32,
    Render = 1 << Component::Render as u32,
}
