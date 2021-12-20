use crate::collision::{Collider, PhysicsLayers};
use crate::shared::Materials;
use crate::steerer::SteerMove;
use bevy::prelude::*;
use core::f32::consts::TAU;

const SPEED: f32 = 6.0;
const ROTATION_SPEED: f32 = TAU / 80.0;
const SIZE: (f32, f32) = (8.0, 8.0);

pub struct Player;

fn spawn(mut commands: Commands, materials: Res<Materials>) {
    let transform = Transform::from_xyz(80.0, 20.0, 0.0);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.plane_material.clone(),
            transform,
            ..Default::default()
        })
        .insert(Player)
        .insert(SteerMove::new(SPEED, Vec3::Y))
        .insert(Collider {
            size: Vec2::new(SIZE.0, SIZE.1),
            layer: PhysicsLayers::PLAYER,
        });
}

fn player_turning(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    for mut transform in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.rotate(Quat::from_rotation_z(ROTATION_SPEED));
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.rotate(Quat::from_rotation_z(-ROTATION_SPEED));
        }
    }
}

fn player_screen_wrap(mut players: Query<&mut Transform, With<Player>>, windows: Res<Windows>) {
    let mut transform = players.iter_mut().nth(0).expect("There are no players!");
    let mut pos = transform.translation;
    let window = windows.get_primary().unwrap();
    let (screen_width, screen_height) = (window.width(), window.height());
    if pos.x < -screen_width / 2.0 {
        pos.x = screen_width / 2.0;
    } else if pos.x >= screen_width / 2.0 {
        pos.x = -screen_width / 2.0;
    }

    if pos.y < -screen_height / 2.0 {
        pos.y = screen_height / 2.0;
    } else if pos.y >= screen_height / 2.0 {
        pos.y = -screen_height / 2.0;
    }
    println!("{} : {}", transform.translation, pos);

    transform.translation = pos;
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("prelude", spawn.system())
            .add_system(player_turning.system())
            .add_system(player_screen_wrap.system());
    }
}
