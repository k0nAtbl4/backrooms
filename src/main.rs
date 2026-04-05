use avian3d::prelude::*;
use bevy::prelude::*;

mod components;
mod temp;
mod systems;

// Импортируем нужные типы и функции
use crate::components::Player;
use crate::systems::camera::{camera_follow_system, camera_rotation_system};
use crate::temp::setup;
use crate::systems::player::player_movement_system;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,            // Стандартные плагины Bevy (рендер, окно и т.д.)
            PhysicsPlugins::default(), // Физика Avian
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement_system, camera_follow_system,camera_rotation_system))
        .run();
}
