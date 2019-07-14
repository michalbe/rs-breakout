use crate::components::com_transform::Transform;

const MAX_ENTITIES: usize = 10000;

pub struct Game {
    pub world: [i32; MAX_ENTITIES],

    // Components here
    pub transform: [Transform; MAX_ENTITIES]
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: [0; MAX_ENTITIES],
            transform: [Transform::empty(); MAX_ENTITIES]
        }
    }
}
