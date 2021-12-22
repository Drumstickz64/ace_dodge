use super::common::SPRITE_ROTATION_OFFSET;
use super::common::{Enemy, SIZE};
use crate::collision::Collider;
use crate::collision::PhysicsLayers;
use crate::math::angle_between_vec3;
use crate::math::random_position_at_edge_of_screen;
use crate::player::Player;
use crate::shared::{Materials, Score};
use crate::steerer::SteerMove;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

const SPEED: f32 = 4.5;
const SPAWN_TIMESTEP: f64 = 1.0;
const ROTATION_SLERP_AMOUNT: f32 = 0.05;

struct RedEnemy;

struct Stats {
    speed_multiplier: f32,
    amount_per_spawn: u32,
}

impl Stats {
    fn default() -> Stats {
        Stats {
            speed_multiplier: 1.0,
            amount_per_spawn: 1,
        }
    }
}

fn spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    windows: Res<Windows>,
    stats: Res<Stats>,
) {
    let window = windows.get_primary().unwrap();

    for _ in 0..stats.amount_per_spawn {
        let position = random_position_at_edge_of_screen(window.width(), window.height());
        let translation = Vec3::new(position.0, position.1, 0.0);
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.red_enemy_material.clone(),
                transform: Transform::from_translation(translation),
                ..Default::default()
            })
            .insert(SteerMove::new(SPEED * stats.speed_multiplier, Vec3::Y))
            .insert(Collider {
                size: Vec2::new(SIZE.0, SIZE.1),
                layer: PhysicsLayers::ENEMY,
            })
            .insert(Enemy)
            .insert(RedEnemy);
    }
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
        let target_rotation = Quat::from_rotation_z(angle + SPRITE_ROTATION_OFFSET);
        let dot_product = enemy_transform.rotation.dot(target_rotation);
        let rotation_direction = if dot_product >= 0.0 { 1.0 } else { -1.0 };
        enemy_transform.rotation = Quat::slerp(
            enemy_transform.rotation * rotation_direction,
            target_rotation,
            ROTATION_SLERP_AMOUNT,
        );
    }
}

fn update_stats(mut stats: ResMut<Stats>, score: Res<Score>) {
    stats.speed_multiplier = f32::min(1.0 + score.0 as f32 * 0.01875, 1.5);
    stats.amount_per_spawn = u32::min(1 + (score.0 as f32 * 0.025) as u32, 3);
}

pub struct RedEnemyPlugin;

impl Plugin for RedEnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Stats::default())
            .add_system(
                spawn
                    .system()
                    .with_run_criteria(FixedTimestep::step(SPAWN_TIMESTEP)),
            )
            .add_system(update_stats.system())
            .add_system(turn_to_player.system());
    }
}
