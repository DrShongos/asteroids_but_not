use bevy::prelude::*;

mod asteroid;
mod game_assets;
mod player;
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
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.projection.scale = 0.25; // TODO: Remove this and make all objects larger instead

    commands.spawn(camera2d_bundle);
}
