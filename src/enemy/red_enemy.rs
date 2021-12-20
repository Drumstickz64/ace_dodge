use super::common::SPRITE_ROTATION_OFFSET;
use super::common::{Enemy, SIZE};
use crate::collision::Collider;
use crate::collision::PhysicsLayers;
use crate::math::angle_between_vec3;
use crate::math::calculate_vec3_at_edge_of_screen;
use crate::player::Player;
use crate::shared::Materials;
use crate::steerer::SteerMove;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

const SPEED: f32 = 4.5;
const SPAWN_TIMESTEP: f64 = 1.0;

struct RedEnemy;

fn spawn(mut commands: Commands, materials: Res<Materials>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let translation = calculate_vec3_at_edge_of_screen(window.width(), window.height());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.red_enemy_material.clone(),
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .insert(SteerMove::new(SPEED, Vec3::Y))
        .insert(Collider {
            size: Vec2::new(SIZE.0, SIZE.1),
            layer: PhysicsLayers::ENEMY,
        })
        .insert(Enemy)
        .insert(RedEnemy);
}

fn turn_to_player(
    mut q: QuerySet<(
        Query<&mut Transform, With<RedEnemy>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    let player_transform = q
        .q1()
        .iter()
        .nth(0)
        .expect("There are no players!")
        .to_owned();
    for mut enemy_transform in q.q0_mut().iter_mut() {
        let angle = angle_between_vec3(player_transform.translation, enemy_transform.translation);
        enemy_transform.rotation = Quat::from_rotation_z(angle + SPRITE_ROTATION_OFFSET);
    }
}

pub struct RedEnemyPlugin;

impl Plugin for RedEnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            spawn
                .system()
                .with_run_criteria(FixedTimestep::step(SPAWN_TIMESTEP)),
        )
        .add_system(turn_to_player.system());
    }
}