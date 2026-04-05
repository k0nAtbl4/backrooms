use crate::components::{MainCamera, Player};
use bevy::prelude::*;

pub fn camera_follow_system(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
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
