use crate::shared::{Materials, Rotate2};
use crate::steerer::SteerMove;
use bevy::prelude::*;

struct Plane;

fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.plane_material.clone(),
            transform: Transform::from_xyz(80.0, 20.0, 10.0),
            ..Default::default()
        })
        .insert(Plane)
        .insert(Rotate2 { speed: 0.08 })
        .insert(SteerMove::new(6.0, Vec3::Y));
}

fn player_turning(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Transform, &Rotate2), With<Plane>>,
) {
    for (mut transform, rotate) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.rotate(Quat::from_rotation_z(rotate.speed));
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.rotate(Quat::from_rotation_z(-rotate.speed));
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("game_setup", spawn_player.system())
            .add_system(player_turning.system());
    }

    fn name(&self) -> &str {
        "PlayerPlugin"
    }
}
