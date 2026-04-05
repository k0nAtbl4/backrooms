use crate::components::{MainCamera3POV, Player};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

pub fn camera_follow_system(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera3POV>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera3POV>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    // Камера следует за игроком с небольшим смещением
    let offset = Vec3::new(0.0, 2.5, -5.0);

    // Поворачиваем смещение вместе с игроком
    let rotated_offset = player_transform.rotation * offset;

    // Устанавливаем позицию камеры
    camera_transform.translation = player_transform.translation + rotated_offset;

    // Камера смотрит на игрока
    camera_transform.look_at(player_transform.translation, Vec3::Y);
}

pub fn camera_rotation_system(
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    for ((mut transform, mut player)) in query.iter_mut() {
        let y_rotation = player.rotation.y - mouse_motion.delta.x * player.sensitivity;  // y - горизонтальная ось
        let x_rotation = player.rotation.x + mouse_motion.delta.y * player.sensitivity;  // x - вертикальная ось
        
        // -0.63 and 0.48
        if x_rotation > 0.48 || x_rotation < -0.63 {
            return;
        }

        transform.rotation = Quat::from_euler(EulerRot::YXZ, y_rotation, x_rotation, 0.0);
        player.rotation.y = y_rotation;
        player.rotation.x = x_rotation;
    }
}
