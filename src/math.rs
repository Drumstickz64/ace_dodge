use bevy::math::Vec3;
use rand::{thread_rng, Rng};

pub fn angle_between_vec3(target: Vec3, start: Vec3) -> f32 {
    let vec_between = target - start;
    f32::atan2(vec_between.y, vec_between.x)
}

pub fn calculate_vec3_at_edge_of_screen(screen_width: f32, screen_height: f32) -> Vec3 {
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
