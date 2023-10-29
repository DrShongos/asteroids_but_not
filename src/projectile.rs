use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::{collision::Collider, game_assets::GameAssets, world::WrapAround};

#[derive(Component)]
pub struct Projectile {
    direction: Vec2,
    speed: f32,
    range: f32,
}

#[derive(Component)]
pub struct ProjectileShooter {
    pub projectile_speed: f32,
    pub projectile_range: f32,

    pub attack_speed: Timer,
}

#[derive(Event)]
pub struct ShootProjectileEvent {
    pub spawn_position: Vec2,
    pub direction: Vec2,
    pub shooter: Entity,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootProjectileEvent>()
            .add_systems(Update, (Self::shoot_projectile_event, Self::handle_attack_speed))
            .add_systems(FixedUpdate, Self::handle_projectiles);
    }
}

impl ProjectilePlugin {
    fn spawn_projectile(
        position: Vec2,
        direction: Vec2,
        speed: f32,
        range: f32,
        commands: &mut Commands,
        game_assets: &Res<GameAssets>,
    ) {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: position.extend(2.0),
                    scale: vec3(2.0, 2.0, 1.0),
                    ..Default::default()
                },
                texture: game_assets.bullet_handle.clone(),
                ..Default::default()
            },
            WrapAround,
            Projectile {
                direction,
                speed,
                range,
            },
            Collider {
                bounds: vec2(16.0, 16.0),
            },
        ));
    }

    fn shoot_projectile_event(
        mut commands: Commands,
        game_assets: Res<GameAssets>,
        mut shoot_projectiles: EventReader<ShootProjectileEvent>,
        mut shooters: Query<&mut ProjectileShooter>,
    ) {
        for shoot_projectile in shoot_projectiles.iter() {
            // This check exists instead of simply unwrapping because of the possibility of the
            // game crashing if the entity that fires the event dies in the same frame. Not sure
            // how often this can happen, though.
            if let Ok(mut shooter) = shooters.get_mut(shoot_projectile.shooter) {
                if shooter.attack_speed.finished() {
                    Self::spawn_projectile(
                        shoot_projectile.spawn_position,
                        shoot_projectile.direction,
                        shooter.projectile_speed,
                        shooter.projectile_range,
                        &mut commands,
                        &game_assets,
                    );

                    shooter.attack_speed.reset();
                }
            }
        }
    }

    fn handle_projectiles(
        mut commands: Commands,
        mut projectiles: Query<(Entity, &mut Transform, &mut Projectile)>,
        time: Res<Time>,
    ) {
        for (entity, mut transform, mut projectile) in projectiles.iter_mut() {
            let delta = time.delta_seconds();

            transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, projectile.direction);
            let movement_speed = projectile.speed * delta;
            let movement = projectile.direction * movement_speed;

            projectile.range -= movement_speed;

            transform.translation += movement.extend(0.0);

            if projectile.range <= 0.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    fn handle_attack_speed(
        mut projectile_shooters: Query<&mut ProjectileShooter>,
        time: Res<Time>,
    ) {
        for mut shooter in projectile_shooters.iter_mut() {
            shooter.attack_speed.tick(time.delta());
        }
    }
}
