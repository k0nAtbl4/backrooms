use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::{MainCamera3POV, Player};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- Освещение ---
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0_f32.to_radians(),
            45.0_f32.to_radians(),
            0.0,
        )),
    ));

    // --- Пол ---
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
        RigidBody::Static,
        Collider::cuboid(100.0, 0.3, 100.0),
    ));

    // --- Игрок ---
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.5, 0.8),
            metallic: 0.7,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.0),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(), // ← явно добавляем компонент скорости
        Player {
            speed: 15.0,
            sensitivity: 0.01,
            rotation: Vec3::ZERO,
        },
    ));

    // --- Камера ---
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        MainCamera3POV{
        },
    ));
}
