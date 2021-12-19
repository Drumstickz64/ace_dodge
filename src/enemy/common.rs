use bevy::app::AppExit;
use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::collision::Collider;
use crate::player::Player;

pub const SIZE: (f32, f32) = (16.0, 16.0);

pub struct Enemy;

fn collision_with_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &GlobalTransform, &Collider), With<Enemy>>,
) {
    for (enemy, transform, collider) in enemies.iter() {
        let other_enemies = enemies.iter().filter(|(item, ..)| *item != enemy);
        for (other_enemy, other_transform, other_collider) in other_enemies {
            if collider.layer != other_collider.layer {
                continue;
            }

            let has_collided_with_enemy = collide(
                transform.translation,
                collider.size,
                other_transform.translation,
                other_collider.size,
            )
            .is_some();

            if has_collided_with_enemy {
                commands.entity(enemy).despawn();
                commands.entity(other_enemy).despawn();
            };
        }
    }
}

// collision with player handled by enemy, ideally this would be handled by a collision system
// but bevy doesn't have one yet and I'm to lazy to make one
fn collision_with_player(
    enemies: Query<(&GlobalTransform, &Collider), With<Enemy>>,
    players: Query<(&GlobalTransform, &Collider), With<Player>>,
    mut exit_writer: EventWriter<AppExit>,
) {
    let (player_transform, player_collider) = players.iter().nth(0).expect("There are no players!");
    for (enemy_transform, enemy_collider) in enemies.iter() {
        let has_collided_with_player = collide(
            enemy_transform.translation,
            enemy_collider.size,
            player_transform.translation,
            player_collider.size,
        )
        .is_some();
        if has_collided_with_player {
            exit_writer.send(AppExit);
        }
    }
}

pub struct CommonEnemyPlugin;

impl Plugin for CommonEnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision_with_enemies.system())
            .add_system(collision_with_player.system());
    }
}
