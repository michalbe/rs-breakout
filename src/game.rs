use crate::{
    blueprints::blu_common::Blueprint,
    components::com_transform::Transform,
};

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

   pub fn add (&mut self, blueprint: &mut Blueprint) -> usize {
        let entity = self.create_entity(2);
        let transform_mixin = Transform::new(blueprint.translation, blueprint.rotation, blueprint.scale);
        transform_mixin(self, entity);

        for mixin in blueprint.using.iter_mut() {
            mixin(self, entity);
        }

        entity
   }
}
