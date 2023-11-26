use bevy::{prelude::*, render::camera::ScalingMode};

mod asteroid;
mod collision;
mod game_assets;
mod player;
mod projectile;
mod ship;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            game_assets::GameAssetsPlugin,
            ship::ShipPlugin,
            player::PlayerPlugin,
            world::WorldPlugin,
            asteroid::AsteroidPlugin,
            collision::CollisionPlugin,
            projectile::ProjectilePlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 1280.0, height: 720.0 };

    commands.spawn(camera2d_bundle);
}
