use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{asteroid::Asteroid, player::Player};

#[derive(Component)]
pub struct Collider {
    pub bounds: Vec2,
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub other_entity: Entity,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Update, Self::check_colliders)
            .add_systems(Update, Self::ship_asteroid_collision);
    }
}

impl CollisionPlugin {
    fn check_colliders(
        colliders: Query<(Entity, &Transform, &Collider)>,
        mut collision_event: EventWriter<CollisionEvent>,
    ) {
        let colliders_iter = colliders.iter();
        for (entity, transform, collider) in colliders_iter {
            // Peeks at the next collider in the query to check collision with them
            // I have no idea why it works flawlessly without skipping entities but it does.
            // However, as a side effect of this approach it is impossible to predict which entity
            // is sent into the event
            let mut peekable_other = colliders.iter().peekable();
            let next = peekable_other.peek();

            if let Some((other_entity, other_transform, other_collider)) = next {
                if collide(
                    transform.translation,
                    collider.bounds,
                    other_transform.translation,
                    other_collider.bounds,
                )
                .is_some()
                    && other_entity != &entity
                {
                    collision_event.send(CollisionEvent {
                        entity: *other_entity,
                        other_entity: entity,
                    });
                }
            }
        }
    }

    fn ship_asteroid_collision(
        mut commands: Commands,
        asteroids: Query<&Asteroid>,
        players: Query<&Player>,
        mut collision_events: EventReader<CollisionEvent>,
    ) {
        for collision_event in &mut collision_events {
            let player_entity = if players.contains(collision_event.entity) {
                collision_event.entity
            } else {
                collision_event.other_entity
            };

            let asteroid_entity = if asteroids.contains(collision_event.entity) {
                collision_event.entity
            } else {
                collision_event.other_entity
            };

            if players.contains(player_entity) && asteroids.contains(asteroid_entity) {
                commands.entity(player_entity).despawn_recursive();
            }
        }
    }
}
