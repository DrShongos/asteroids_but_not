use bevy::{math::vec2, prelude::*};
use rand::Rng;

use crate::{collision::Collider, game_assets::GameAssets, world::WrapAround};

#[derive(Component)]
pub struct Asteroid {
    rotation_speed: f32,
    velocity: Vec2,

    pub tier: u16,
    pub hits: u16,
}

#[derive(Event)]
pub struct AsteroidDestructionEvent {
    pub entity: Entity,
    pub asteroid_tier: u16,
    pub destruction_point: Vec3,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AsteroidDestructionEvent>()
            .add_systems(Startup, Self::populate_with_asteroids)
            .add_systems(FixedUpdate, Self::asteroid_movement)
            .add_systems(Update, Self::asteroid_destruction_event);
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
            vec2(300., 30.),
            &game_assets,
            0,
        );
        spawn_asteroid(
            &mut commands,
            6,
            4.0,
            450.,
            vec2(0.45, -0.35),
            vec2(-320., 30.),
            &game_assets,
            1,
        );
        spawn_asteroid(
            &mut commands,
            2,
            15.0,
            1000.,
            vec2(0.15, 0.24),
            vec2(200., -30.),
            &game_assets,
            2,
        );
    }

    fn asteroid_movement(mut asteroid_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
        for (mut transform, asteroid) in asteroid_query.iter_mut() {
            let delta = time.delta_seconds();
            let movement = (asteroid.velocity * delta).extend(0.0);

            transform.translation += movement;
            transform.rotate_z(asteroid.rotation_speed * delta);
        }
    }

    fn asteroid_destruction_event(
        mut commands: Commands,
        mut asteroid_destruction: EventReader<AsteroidDestructionEvent>,
        game_assets: Res<GameAssets>,
        atlases: Res<Assets<TextureAtlas>>,
    ) {

        let asteroid_variants = atlases.get(&game_assets.asteroid_handle).unwrap().len();
        
        for event in asteroid_destruction.iter() {
            let num_splits = if event.asteroid_tier >= 6 {
                4
            } else if event.asteroid_tier > 1 {
                2
            } else {
                0
            };

            commands.entity(event.entity).despawn_recursive();

            let mut rng = rand::thread_rng();


            for _ in 0..num_splits {
                let direction = vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
                let speed = rng.gen_range(50.0..100.0) * event.asteroid_tier as f32;
                let asteroid_index = rng.gen_range(0..asteroid_variants);

                let rotation_speed = rng.gen_range(1.0..15.0);

                spawn_asteroid(
                    &mut commands,
                    event.asteroid_tier - 1,
                    rotation_speed,
                    speed,
                    direction,
                    event.destruction_point.truncate(),
                    &game_assets,
                    asteroid_index,
                );
            }
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
            rotation_speed,
            velocity: direction * speed,
            tier,
            hits: tier,
        },
        Collider {
            bounds: vec2(16.0 * (tier as f32 / 1.25), 16.0 * (tier as f32 / 1.25)),
        },
    ));
}
