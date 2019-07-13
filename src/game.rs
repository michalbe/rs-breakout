const MAX_ENTITIES: usize = 10000;

pub struct Game {
    world: [i32; MAX_ENTITIES],
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: [0; MAX_ENTITIES]
        }
    }
}
