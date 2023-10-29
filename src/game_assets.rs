use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub ship1_handle: Handle<TextureAtlas>,
    pub asteroid_handle: Handle<TextureAtlas>,
    pub bullet_handle: Handle<Image>,
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::load_graphics);
    }
}

impl GameAssetsPlugin {
    fn load_graphics(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let ship1_handle = Self::load_atlas(
            "ship1.png",
            Vec2::new(16.0, 16.0),
            2,
            1,
            &asset_server,
            &mut texture_atlases,
        );
        let asteroid_handle = Self::load_atlas(
            "meteorites.png",
            Vec2::new(16.0, 16.0),
            3,
            1,
            &asset_server,
            &mut texture_atlases,
        );

        let bullet_handle = asset_server.load("laser-bullet.png");

        let game_assets = GameAssets {
            ship1_handle,
            asteroid_handle,
            bullet_handle,
        };

        commands.insert_resource(game_assets);
    }

    fn load_atlas(
        name: &str,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        asset_server: &ResMut<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let image = asset_server.load(name);
        let atlas = TextureAtlas::from_grid(image, tile_size, columns, rows, None, None);
        let handle = texture_atlases.add(atlas);

        handle
    }
}
