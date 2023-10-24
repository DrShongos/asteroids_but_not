use bevy::{math::vec2, prelude::*};

use crate::{game_assets::GameAssets, world::WrapAround};

#[derive(Component)]
pub struct Asteroid {
    direction: Vec2,
    speed: f32,

    rotation_speed: f32,
    velocity: Vec2,

    tier: u16,
    hits: u16,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Self::populate_with_asteroids)
            .add_systems(FixedUpdate, Self::asteroid_movement);
    }
}

impl AsteroidPlugin {
    /// A temporary system that spawns 3 asteroids to test their behaviour
    fn populate_with_asteroids(mut commands: Commands, game_assets: Res<GameAssets>) {
        spawn_asteroid(
            &mut commands,
            4,
            7.0,
            750.,
            vec2(-0.45, 0.35),
            vec2(80., 3.),
            &game_assets,
            0,
        );
        spawn_asteroid(
            &mut commands,
            6,
            4.0,
            450.,
            vec2(0.45, -0.35),
            vec2(-80., 3.),
            &game_assets,
            1,
        );
        spawn_asteroid(
            &mut commands,
            2,
            15.0,
            1000.,
            vec2(0.15, 0.24),
            vec2(8., -30.),
            &game_assets,
            2,
        );
    }

    fn asteroid_movement(
        mut asteroid_query: Query<(&mut Transform, &Asteroid)>,
        time: Res<Time>,
    ) {
        for (mut transform, asteroid) in asteroid_query.iter_mut() {
            let delta = time.delta_seconds();
            let movement = (asteroid.velocity * delta).extend(0.0);

            transform.translation += movement;
            transform.rotate_z(asteroid.rotation_speed * delta);
        }
    }
}

pub fn spawn_asteroid(
    commands: &mut Commands,
    tier: u16,
    rotation_speed: f32,
    speed: f32,
    direction: Vec2,
    position: Vec2,
    game_assets: &Res<GameAssets>,
    sprite_index: usize,
) {
    let asteroid_atlas = &game_assets.asteroid_handle;

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: position.extend(1.0),
                scale: Vec3::new(tier as f32, tier as f32, 1.0),
                ..Default::default()
            },
            texture_atlas: asteroid_atlas.clone(),
            sprite: TextureAtlasSprite::new(sprite_index),
            ..Default::default()
        },
        WrapAround,
        Asteroid {
            direction,
            speed,
            rotation_speed,
            velocity: direction * speed,
            tier,
            hits: tier,
        },
    ));
}
