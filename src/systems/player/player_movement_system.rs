use bevy::prelude::*;

use crate::components::{MainCamera, Player};

use avian3d::prelude::*;

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
    for (mut player_transform, player) in player_query.iter_mut() {
        let cam = match camera_query.single() {
            Ok(cam) => cam,
            Err(e) => Err(format!("Error retrieving camera: {} ", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += cam.forward().as_vec3();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += cam.back().as_vec3();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += cam.left().as_vec3();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += cam.right().as_vec3();
        }

        direction.y = 0.0; // ignore camera angle for moving up/down


        if keyboard_input.pressed(KeyCode::Space) {
            direction += cam.up().as_vec3(); // need to rewrite
        }
        let movement = direction.normalize_or_zero() * player.speed * time.delta_secs();

        player_transform.translation += movement;
    }
}
