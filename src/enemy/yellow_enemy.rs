use super::common::SPRITE_ROTATION_OFFSET;
use super::common::{Enemy, SIZE};
use crate::collision::Collider;
use crate::collision::PhysicsLayers;
use crate::math::angle_between_vec3;
use crate::math::calculate_vec3_at_edge_of_screen;
use crate::shared::Materials;
use crate::steerer::SteerMove;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

const SPEED: f32 = 8.5;
const SPAWN_TIMESTEP: f64 = 3.0;

struct YellowEnemy;

fn spawn(mut commands: Commands, materials: Res<Materials>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let translation = calculate_vec3_at_edge_of_screen(window.width(), window.height());
    let angle_to_center = angle_between_vec3(Vec3::ZERO, translation);
    let rotation = Quat::from_rotation_z(angle_to_center + SPRITE_ROTATION_OFFSET);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.yellow_enemy_material.clone(),
            transform: Transform {
                translation,
                rotation,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SteerMove::new(SPEED, Vec3::Y))
        .insert(Collider {
            size: Vec2::new(SIZE.0, SIZE.1),
            layer: PhysicsLayers::ENEMY,
        })
        .insert(Enemy)
        .insert(YellowEnemy);
}

pub struct YellowEnemyPlugin;

impl Plugin for YellowEnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            spawn
                .system()
                .with_run_criteria(FixedTimestep::step(SPAWN_TIMESTEP)),
        );
    }
}