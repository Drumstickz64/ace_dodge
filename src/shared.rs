use bevy::prelude::*;

/// A global store of all materials used in the game
pub struct Materials {
    pub plane_material: Handle<ColorMaterial>,
    pub red_enemy_material: Handle<ColorMaterial>,
    pub yellow_enemy_material: Handle<ColorMaterial>,
}
