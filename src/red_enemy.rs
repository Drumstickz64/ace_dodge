use crate::player::Player;
use crate::shared::Materials;
use crate::steerer::SteerMove;
use bevy::prelude::*;
use rand::prelude::{thread_rng, Rng};

const SPEED: f32 = 4.5;

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
        .insert(RedEnemy);
}

fn calculate_spawn_position(screen_width: f32, screen_height: f32) -> Vec3 {
    let mut rng = thread_rng();
    let edge = rng.gen_range(0..4);

    let (x, y) = match edge {
        0 => (0.0, rng.gen_range(0.0..screen_height)), // left
        1 => (rng.gen_range(0.0..screen_width), 0.0),  // up
        2 => (screen_width, rng.gen_range(0.0..screen_height)), // right
        3 => (rng.gen_range(0.0..screen_width), screen_height), // down
        _ => panic!("edge should be between 0 and 4"),
    };
    Vec3::new(x, y, 10.0)
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
        enemy_transform.look_at(player_transform.translation, Vec3::Z);
    }
}

pub struct RedEnemyPlugin;

impl Plugin for RedEnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("prelude", spawn.system())
            .add_system(turn_to_player.system());
    }
}
