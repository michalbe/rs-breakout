use crate::components::com_collide::Collision;
use crate::math::vec2::Vec2;
use crate::components::com_transform2d::Transform2d;
use crate::game::MAX_ENTITIES;
use crate::game::Game;
use crate::components::Has;
use crate::components::com_collide::Collide;
use crate::math::mat2d::Mat2d;

const QUERY: u32 = Has::Transform2d as u32 | Has::Collide as u32;

pub fn sys_collide(game: &mut Game, delta: f32) {
    let mut all_colliders: Vec<Collide> = Vec::new();
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            if let (
                Some(transform),
                Some(mut collider)
            ) = (
                game.transform[i],
                game.collide[i]
            ) {
                collider.collision = None;
                compute_aabb(transform, &mut collider);
                game.collide[i] = Some(collider);
                all_colliders.push(collider);
            }
        }
    }

    for i in 0..all_colliders.len() {
        let mut collider = all_colliders[i];
        for j in 0..all_colliders.len() {
            let other = all_colliders[j];
            if collider.entity != other.entity && intersect_aabb(&collider, &other) {
                collider.collision = Some(Collision {
                    entity: other.entity,
                    hit: calculate_penetration(&collider, &other)
                });

                // TODO: ...
                game.collide[collider.entity] = Some(collider);
            }
        }
    }
}

fn compute_aabb(transform: Transform2d, collide: &mut Collide) {
    collide.center = Mat2d::get_translation(transform.world);
    collide.min.x = collide.center.x - collide.size.x / 2.0;
    collide.min.y = collide.center.y - collide.size.y / 2.0;
    collide.max.x = collide.center.x + collide.size.x / 2.0;
    collide.max.y = collide.center.y + collide.size.y / 2.0;
}

fn intersect_aabb(a: &Collide, b: &Collide) -> bool {
    a.min.x < b.max.x
        && a.max.x > b.min.x
        && a.min.y < b.max.y
        && a.max.y > b.min.y
}

fn calculate_penetration(a: &Collide, b: &Collide) -> Vec2 {
    let mut penetration = Vec2::empty();
    let distance_x = a.center.x - b.center.x;
    let penetration_x = a.size.x / 2.0 + b.size.x / 2.0 - distance_x.abs();

    let distance_y = a.center.y - b.center.y;
    let penetration_y = a.size.y / 2.0 + b.size.y / 2.0 - distance_y.abs();

    if penetration_x < penetration_y {
        penetration.x = penetration_x * distance_x.signum();
        penetration.y = 0.0;
    } else {
        penetration.x = 0.0;
        penetration.y = penetration_y * distance_y.signum();
    }

    penetration
}
