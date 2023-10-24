use crate::{collision::Collider, game_assets::GameAssets, player::Player, world::WrapAround};
use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

#[derive(Component)]
pub struct Ship {
    friction: f32,
    acceleration: f32,
    speed: f32,

    pub rotation_speed: f32,
    pub moving: bool,

    velocity: Vec2,
}

impl Ship {
    pub fn new(friction: f32, acceleration: f32, speed: f32, rotation_speed: f32) -> Ship {
        Ship {
            friction,
            acceleration,
            speed,
            rotation_speed,
            moving: false,
            velocity: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
pub struct ShipInput {
    pub forward: bool,
    pub rotate: bool,
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_player_ship)
            .add_systems(FixedUpdate, Self::ship_movement)
            .add_systems(Update, Self::ship_animation);
    }
}

impl ShipPlugin {
    fn spawn_player_ship(mut commands: Commands, game_assets: Res<GameAssets>) {
        let ship_atlas = &game_assets.ship1_handle;

        commands.spawn((
            SpriteSheetBundle {
                transform: Transform {
                    translation: vec3(0., 0., 0.),
                    scale: vec3(4., 4., 4.),
                    ..Default::default()
                },
                texture_atlas: ship_atlas.clone(),
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            },
            Ship::new(2., 8., 5., 2.),
            Player,
            WrapAround,
            Collider {
                bounds: vec2(16.0 * 4.0, 16.0 * 4.0),
            },
        ));
    }

    fn ship_movement(mut ship_query: Query<(&mut Transform, &mut Ship)>, time: Res<Time>) {
        for (mut transform, mut ship) in ship_query.iter_mut() {
            let delta = time.delta_seconds();

            // Get the ship's direction from rotation
            let direction = transform.rotation * Vec3::Y;

            if ship.moving {
                ship.velocity = ship
                    .velocity
                    .lerp(Vec2::new(ship.speed, ship.speed), ship.acceleration * delta);
            } else {
                ship.velocity = ship.velocity.lerp(Vec2::ZERO, ship.friction * delta);
            }

            // Turn the ship's movement into vec3 so that it can be applied
            let movement = direction * ship.velocity.extend(0.);

            transform.translation += movement;
        }
    }

    fn ship_animation(mut ship_query: Query<(&Ship, &mut TextureAtlasSprite)>) {
        for (ship, mut sprite) in ship_query.iter_mut() {
            if ship.moving {
                sprite.index = 1;
            } else {
                sprite.index = 0;
            }
        }
    }
}
