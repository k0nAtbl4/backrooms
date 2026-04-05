use bevy::prelude::*;

#[derive(Component)]
pub struct Player{
    pub speed: f32,        // скорость движения
    pub sensitivity: f32,  // чувствительность мыши (потом пригодится)
    pub rotation: Vec3, // угол поворота
    pub animation_state: PlayerAnimationStates,
}


#[derive(Default)]
pub enum PlayerAnimationStates{
    #[default]
    Idle,
    Walking,
    Run,
    DownRun,
    DownWalking,
    DownIdle,
    BackWalking,
    Jumping,
}