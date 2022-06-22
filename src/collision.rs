use bevy::prelude::*;

pub type CollisionCallback = fn(Commands, World, (Collider, Entity), (Collider, Entity));

#[derive(Component)]
pub struct Collider {
    pub collision_callback: CollisionCallback,
}

impl Default for Collider {
    fn default() -> Self {
        Collider {
            collision_callback: |_, _, _, _| {},
        }
    }
}

pub fn collision_check(vector_a: Vec3, vector_b: Vec3, distance: f32) -> bool {
    return distance > vector_a.distance(vector_b);
}

#[test]
#[cfg(test)]
fn collison_check_test_1() {

    let vector_a = Vec3::new(0.0, 0.0, 0.0);
    let vector_b = Vec3::new(0.5, 0.0, 0.0);

    let collides = collision_check(vector_a, vector_b, 1.0);

    // Check resulting changes
    assert_eq!(collides, true);
}



#[test]
#[cfg(test)]
fn collison_check_test_2() {

    let vector_a = Vec3::new(0.0, 0.0, 0.0);
    let vector_b = Vec3::new(-0.5, 0.0, 0.0);

    let collides = collision_check(vector_a, vector_b, 1.0);

    // Check resulting changes
    assert_eq!(collides, true);
}



#[test]
#[cfg(test)]
fn collison_check_test_3() {

    let vector_a = Vec3::new(0.0, 0.0, 0.0);
    let vector_b = Vec3::new(0.0, 0.0, 0.0);

    let collides = collision_check(vector_a, vector_b, 1.0);

    // Check resulting changes
    assert_eq!(collides, true);
}

#[test]
#[cfg(test)]
fn collison_check_test_4() {

    let vector_a = Vec3::new(0.0, 0.0, 0.0);
    let vector_b = Vec3::new(1.1, 0.0, 0.0);

    let collides = collision_check(vector_a, vector_b, 1.0);

    // Check resulting changes
    assert_eq!(collides, false);
}


#[test]
#[cfg(test)]
fn collison_check_test_5() {

    let vector_a = Vec3::new(0.0, 0.0, 0.0);
    let vector_b = Vec3::new(1.0, 1.0, 0.0);

    let collides = collision_check(vector_a, vector_b, 1.0);

    // Check resulting changes
    assert_eq!(collides, false);
}
