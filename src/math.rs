use bevy::math::Vec3;

pub fn angle_between_vec3(target: Vec3, start: Vec3) -> f32 {
    let vec_between = target - start;
    f32::atan2(vec_between.y, vec_between.x)
}
