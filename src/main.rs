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
    let camera2d_bundle = Camera2dBundle::default();

    commands.spawn(camera2d_bundle);
}
