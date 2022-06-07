use bevy::math::Vec3;
use rand::Rng;

/**
 * Gets random float in a given range.
 */
pub fn range_f32(min: f32, max: f32) -> f32 {
    return min + ((max - min) * rand::random::<f32>());
}

/**
 * Gets random int in a given range.
 */
pub fn range_i32(min: i32, max: i32) -> i32 {
    return rand::thread_rng().gen_range(min..max);
}

/**
 * Gets a random vector. All components will be randomised between +/- radius from origin.
 */
pub fn vec3(radius: f32) -> Vec3 {
    return Vec3::new(
        range_f32(-radius, radius),
        range_f32(-radius, radius),
        range_f32(-radius, radius),
    );
}

pub fn from_vec(vec: &Vec<String>) -> String {
    let length = vec.len() as i32;
    let pick = range_i32(0, length) as usize;
    return vec[pick].clone();
}
