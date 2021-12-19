use std::f32::consts::TAU;

use super::common::{Enemy, SIZE};
use crate::collision::Collider;
use crate::collision::PhysicsLayers;
use crate::math::angle_between_vec3;
use crate::player::Player;
use crate::shared::Materials;
use crate::steerer::SteerMove;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::{thread_rng, Rng};

const SPEED: f32 = 4.5;
const SPAWN_TIMESTEP: f64 = 1.0;

struct RedEnemy;

fn spawn(mut commands: Commands, materials: Res<Materials>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let position = calculate_spawn_position(window.width(), window.height());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.red_enemy_material.clone(),
            transform: Transform::from_translation(position),
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

fn calculate_spawn_position(screen_width: f32, screen_height: f32) -> Vec3 {
    let mut rng = thread_rng();
    let edge = rng.gen_range(0..4);

    // TODO: Turn this into an enum or struct const fields
    let (x, y) = match edge {
        // left
        0 => (
            -screen_width / 2.0,
            rng.gen_range(-screen_height / 2.0..screen_height / 2.0),
        ),
        // up
        1 => (
            -rng.gen_range(-screen_width / 2.0..screen_width / 2.0),
            -screen_height / 2.0,
        ),
        // right
        2 => (
            screen_width / 2.0,
            rng.gen_range(-screen_height / 2.0..screen_height / 2.0),
        ),
        // down
        3 => (
            -rng.gen_range(-screen_width / 2.0..screen_width / 2.0),
            screen_height / 2.0,
        ),
        _ => panic!("edge should be between 0 and 4"),
    };
    Vec3::new(x, y, 0.0)
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
        let offset = -TAU / 4.0;
        enemy_transform.rotation = Quat::from_rotation_z(angle + offset);
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
