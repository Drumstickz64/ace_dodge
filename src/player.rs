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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("prelude", spawn.system())
            .add_system(player_turning.system());
    }
}
