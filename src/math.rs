use bevy::math::Vec3;
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

pub fn angle_between_vec3(target: Vec3, start: Vec3) -> f32 {
    let vec_between = target - start;
    f32::atan2(vec_between.y, vec_between.x)
}

enum ScreenEdge {
    Left,
    Up,
    Right,
    Down,
}

impl Distribution<ScreenEdge> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ScreenEdge {
        match rng.gen_range(0..4) {
            0 => ScreenEdge::Left,
            1 => ScreenEdge::Up,
            2 => ScreenEdge::Right,
            _ => ScreenEdge::Down,
        }
    }
}

pub fn random_position_at_edge_of_screen(screen_width: f32, screen_height: f32) -> (f32, f32) {
    let mut rng = thread_rng();
    let edge: ScreenEdge = rng.gen();

    let (x, y) = match edge {
        ScreenEdge::Left => (
            -screen_width / 2.0,
            rng.gen_range(-screen_height / 2.0..screen_height / 2.0),
        ),
        ScreenEdge::Up => (
            -rng.gen_range(-screen_width / 2.0..screen_width / 2.0),
            -screen_height / 2.0,
        ),
        ScreenEdge::Right => (
            screen_width / 2.0,
            rng.gen_range(-screen_height / 2.0..screen_height / 2.0),
        ),
        ScreenEdge::Down => (
            -rng.gen_range(-screen_width / 2.0..screen_width / 2.0),
            screen_height / 2.0,
        ),
    };
    (x, y)
}
