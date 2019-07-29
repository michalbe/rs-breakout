use crate::{
    blueprints::blu_common::Blueprint,
    components::{
        Components,
        com_transform::Transform,
    }
};

const MAX_ENTITIES: usize = 100;

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

    fn create_entity(&mut self, mask: i32) -> usize {
        for i in 0..MAX_ENTITIES {
            if self.world[i] == 0 {
                self.world[i] = mask;
                return i;
            }
        }

        panic!("No more entities available!");
    }

   pub fn add (&mut self, blueprint: &mut Blueprint) -> usize {
        let entity = self.create_entity(Components::Transform as i32);
        let transform_mixin = Transform::new(blueprint.translation, blueprint.rotation, blueprint.scale);
        transform_mixin(self, entity);

        for mixin in blueprint.using.iter_mut() {
            mixin(self, entity);
        }

        entity
   }
}


#[test]
fn game_add_test() {
    use crate::math::{
        vec3::Vec3,
        quat::Quat,
    };

    let mut game = Game::new();
    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::new(0.0, 1.0, 0.0, 0.0);
    let scale = Vec3::new(2.0, 2.0, 2.0);

    let mut blueprint = Blueprint {
        translation: Some(translation),
        rotation: Some(rotation),
        scale: Some(scale),
        using: Vec::new(),
    };

    let entity_1 = game.add(&mut blueprint);
    let entity_2 = game.add(&mut blueprint);

    assert_eq!(entity_1, 0);
    assert_eq!(entity_2, 1);

    for i in vec![entity_1, entity_2] {
        assert_eq!(game.transform[i].translation.x, translation.x, "translation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].translation.y, translation.y, "translation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].translation.z, translation.z, "translation on entity fits the one in blueprint");

        assert_eq!(game.transform[i].rotation.x, rotation.x, "rotation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].rotation.y, rotation.y, "rotation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].rotation.z, rotation.w, "rotation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].rotation.w, rotation.w, "rotation on entity fits the one in blueprint");

        assert_eq!(game.transform[i].scale.x, scale.x, "scale on entity fits the one in blueprint");
        assert_eq!(game.transform[i].scale.y, scale.y, "scale on entity fits the one in blueprint");
        assert_eq!(game.transform[i].scale.z, scale.z, "scale on entity fits the one in blueprint");
    }
}
