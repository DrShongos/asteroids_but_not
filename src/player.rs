use bevy::prelude::*;

use crate::ship::Ship;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::player_input);
    }
}

impl PlayerPlugin {
    fn player_input(
        mut player_query: Query<(&mut Ship, &mut Transform), With<Player>>,
        keys: Res<Input<KeyCode>>,
        time: Res<Time>,
    ) {
        let (mut ship, mut transform) = player_query.single_mut();

        let delta = time.delta_seconds();

        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
            transform.rotate_z(ship.rotation_speed * delta);
        }

        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
            transform.rotate_z(-1. * ship.rotation_speed * delta);
        }

        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            ship.moving = true;
        } else {
            ship.moving = false;
        }
    }
}
