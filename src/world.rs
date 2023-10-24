use bevy::{math::vec2, prelude::*};

#[derive(Component)]
pub struct WrapAround;

#[derive(Resource)]
pub struct WorldBounds(Vec2);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::get_bounds)
            .add_systems(FixedUpdate, Self::wrap_positions_around_world);
    }
}

impl WorldPlugin {
    fn get_bounds(mut commands: Commands) {
        let bounds = WorldBounds(vec2(1280.0 / 1.75, 720.0 / 1.75));

        commands.insert_resource(bounds);
    }

    fn wrap_positions_around_world(
        mut transform_query: Query<&mut Transform, With<WrapAround>>,
        bounds: Res<WorldBounds>,
    ) {
        for mut transform in transform_query.iter_mut() {
            transform.translation.x = wrap(transform.translation.x, -bounds.0.x, bounds.0.x);
            transform.translation.y = wrap(transform.translation.y, -bounds.0.y, bounds.0.y);
        }
    }
}

fn wrap(value: f32, min: f32, max: f32) -> f32 {
    if value <= min {
        return max;
    }

    if value >= max {
        return min;
    }

    value
}
