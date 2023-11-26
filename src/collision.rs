use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    asteroid::{Asteroid, AsteroidDestructionEvent},
    player::Player,
    projectile::Projectile,
};

#[derive(Component)]
pub struct Collider {
    pub bounds: Vec2,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::ship_asteroid_collision,
                Self::projectile_asteroid_collision,
            ),
        );
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

    fn projectile_asteroid_collision(
        mut commands: Commands,
        mut destruction_event: EventWriter<AsteroidDestructionEvent>,
        mut asteroids: Query<(Entity, &Transform, &Collider, &mut Asteroid)>,
        projectiles: Query<(Entity, &Transform, &Collider, &Projectile)>,
    ) {
        for (projectile_entity, projectile_transform, projectile_collider, projectile) in
            projectiles.iter()
        {
            for (asteroid_entity, asteroid_transform, asteroid_collider, mut asteroid) in
                asteroids.iter_mut()
            {
                if collide(
                    projectile_transform.translation,
                    projectile_collider.bounds,
                    asteroid_transform.translation,
                    asteroid_collider.bounds,
                )
                .is_some()
                {
                    asteroid.hits -= 1;
                    commands.entity(projectile_entity).despawn_recursive();

                    if asteroid.hits == 0 {
                        destruction_event.send(AsteroidDestructionEvent {
                            entity: asteroid_entity,
                            asteroid_tier: asteroid.tier,
                            destruction_point: asteroid_transform.translation,
                        })
                    }
                }
            }
        }
    }
}
