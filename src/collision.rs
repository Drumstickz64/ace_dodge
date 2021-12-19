use bevy::prelude::*;

// I will only make box colliders
pub struct Collider {
    pub size: Vec2,
    pub layer: u8,
}

pub struct PhysicsLayers;

impl PhysicsLayers {
    pub const PLAYER: u8 = 1;
    pub const ENEMY: u8 = 1;
}
