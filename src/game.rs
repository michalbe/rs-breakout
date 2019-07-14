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

    pub fn create_entity(&mut self, mask: i32) -> usize {
        for i in 0..MAX_ENTITIES {
            if self.world[i] == 0 {
                self.world[i] = mask;
                return i;
            }
        }

        panic!("No more entities available!");
    }
}
