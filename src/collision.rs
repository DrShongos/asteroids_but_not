use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{asteroid::Asteroid, player::Player};

#[derive(Component)]
pub struct Collider {
    pub bounds: Vec2,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::ship_asteroid_collision);
    }
}

impl CollisionPlugin {
    fn ship_asteroid_collision(
        mut commands: Commands,
        asteroids: Query<(&Transform, &Collider), With<Asteroid>>,
        players: Query<(Entity, &Transform, &Collider), With<Player>>,
    ) {
        for (player, player_transform, player_collider) in players.iter() {
            for (asteroid_transform, asteroid_collider) in asteroids.iter() {
                if collide(
                    player_transform.translation,
                    player_collider.bounds,
                    asteroid_transform.translation,
                    asteroid_collider.bounds,
                )
                .is_some()
                {
                    commands.entity(player).despawn_recursive();
                }
            }
        }
    }
}
