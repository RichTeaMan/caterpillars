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
