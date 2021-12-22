use super::common::SPRITE_ROTATION_OFFSET;
use super::common::{Enemy, SIZE};
use crate::collision::Collider;
use crate::collision::PhysicsLayers;
use crate::math::angle_between_vec3;
use crate::math::random_position_at_edge_of_screen;
use crate::shared::{Materials, Score};
use crate::steerer::SteerMove;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

const SPEED: f32 = 8.5;
const SPAWN_TIMESTEP: f64 = 3.0;

struct YellowEnemy;

#[derive(Default)]
struct Stats {
    amount_per_spawn: u32,
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
}

fn update_stats(mut stats: ResMut<Stats>, score: Res<Score>) {
    if !score.is_changed() {
        return;
    }
    stats.amount_per_spawn = u32::min(1 + (score.0 as f32 * 0.05) as u32, 4);
}

pub struct YellowEnemyPlugin;

impl Plugin for YellowEnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Stats::default())
            .add_system(update_stats.system())
            .add_system(
                spawn
                    .system()
                    .with_run_criteria(FixedTimestep::step(SPAWN_TIMESTEP)),
            );
    }
}
