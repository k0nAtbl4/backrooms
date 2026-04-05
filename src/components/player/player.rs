use std::default;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player{
    pub speed: f32,        // скорость движения
    pub sensitivity: f32,  // чувствительность мыши (потом пригодится)
    pub rotation: Vec3, // угол поворота
}

#[derive(Component, PartialEq)]
pub struct Position{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Component)]
struct Velocity {
    pub dx: f32,
    pub dy: f32,
    pub dz: f32
}
