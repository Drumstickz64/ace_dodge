/// A Steerer is an entity that automatically moves in a given forward direction.
use bevy::prelude::*;

pub struct SteerMove {
    velocity: Vec3,
    /// the direction the front of the Entity is facing when it has no rotation
    front_direction: Vec3,
    speed: f32,
}

impl SteerMove {
    pub fn new(speed: f32, front_direction: Vec3) -> SteerMove {
        SteerMove {
            velocity: front_direction * speed,
            front_direction,
            speed,
        }
    }
}

fn steerer_movement(mut steerers: Query<(&mut Transform, &mut SteerMove)>) {
    for (mut transform, mut steer_move) in steerers.iter_mut() {
        let direction = transform.rotation.mul_vec3(steer_move.front_direction);
        steer_move.velocity = direction * steer_move.speed;
        transform.translation += steer_move.velocity;
    }
}

pub struct SteererPlugin;

impl Plugin for SteererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(steerer_movement.system());
    }
}
