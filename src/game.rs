use crate::{
    blueprints::blu_common::Blueprint,
    components::{
        com_transform2d::Transform2d,
        com_draw2d::Draw2d,
    }
};


const MAX_ENTITIES: usize = 10000;

pub struct Game {
    pub world: Vec<u32>,

    // Components here
    pub transform: Vec<Option<Transform2d>>,
    pub draw2d: Vec<Option<Draw2d>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: vec![0; MAX_ENTITIES],
            transform: vec![None; MAX_ENTITIES],
            draw2d: vec![None; MAX_ENTITIES],
        }
    }

    fn create_entity(&mut self) -> usize {
        for i in 0..MAX_ENTITIES {
            if self.world[i] == 0 {
                return i;
            }
        }

        panic!("No more entities available!");
    }

   pub fn add(&mut self, blueprint: &mut Blueprint) -> usize {
        let entity = self.create_entity();
        let transform_mixin = Transform2d::new(blueprint.translation, blueprint.rotation, blueprint.scale);
        transform_mixin(self, entity);

        for mixin in blueprint.using.iter_mut() {
            mixin(self, entity);
        }

        entity
   }
}


#[test]
fn game_add_test() {
    use crate::{
        math::{
            vec2::Vec2,
        },
        components::{
            Has,
        }
    };

    let mut game = Game::new();
    let translation = Vec2::new(1.0, 2.0);
    let rotation = 0.0;
    let scale = Vec2::new(2.0, 2.0);

    let mut blueprint_without_mixins = Blueprint {
        translation: Some(translation),
        rotation: Some(rotation),
        scale: Some(scale),
        using: vec![],
    };

    let mut blueprint_with_mixins = Blueprint {
        translation: Some(translation),
        rotation: Some(rotation),
        scale: Some(scale),
        using: vec![
            Box::new(Draw2d::new(Some(5.0), Some(5.0), Some([125, 125, 125, 255])))
        ],
    };

    let entity_1 = game.add(&mut blueprint_without_mixins);
    let entity_2 = game.add(&mut blueprint_with_mixins);

    let mask = Has::Transform as u32;
    let mask_with_mixins = Has::Transform as u32 | Has::Draw2d as u32;

    assert_eq!(entity_1, 0, "proper entity index created");
    assert_eq!(entity_2, 1, "proper entity index created");

    for i in vec![entity_1, entity_2] {
        assert_eq!(game.world[i] & mask, mask, "proper entity component mask created");

        assert_eq!(game.transform[i].unwrap().translation.x, translation.x, "translation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().translation.y, translation.y, "translation on entity fits the one in blueprint");

        assert_eq!(game.transform[i].unwrap().rotation, rotation, "rotation on entity fits the one in blueprint");

        assert_eq!(game.transform[i].unwrap().scale.x, scale.x, "scale on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().scale.y, scale.y, "scale on entity fits the one in blueprint");
    }

    assert_eq!(game.draw2d[entity_2].unwrap().width, 5.0, "draw2d properties were properly set by the draw2d mixin");

    match game.draw2d[entity_1] {
        None => {
            assert!(true, "Entity misses components as expected");
        }
        Some(_) => {
            assert!(false, "Entity shouldn't have component if it wasn't attached with mixin");
        }
    }


    assert_eq!(game.world[1] & mask_with_mixins, mask_with_mixins, "proper entity component mask created for entity with mixins");

}
