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
    distance > vector_a.distance(vector_b)
}

#[allow(dead_code)]
pub fn inside_polygon(vector: Vec2, polygon: &[Vec2]) -> bool {
    // This method counts how many times an imaginary line drawn from the vector eastwards
    // overlaps lines of the polygon. An odd number of overlaps is considered inside the polygon,
    // even number (or zero) is not in the polygon.

    let x1 = vector.x;
    let y1 = vector.y;
    let x2 = vector.x + 1_000_000.0;
    let y2 = vector.y;

    let mut lines = Vec::new();
    for i in 0..(polygon.len() - 1) {
        lines.push((polygon[i], polygon[i + 1]));
    }
    lines.push((polygon[polygon.len() - 1_usize], polygon[0]));

    let mut intersections = 0;
    for line in lines {
        let x3 = line.0.x;
        let y3 = line.0.y;
        let x4 = line.1.x;
        let y4 = line.1.y;

        // taken from wikipedia: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
        let t = (((x1 - x3) * (y3 - y4)) - ((y1 - y3) * (x3 - x4)))
            / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));
        let u = (((x1 - x3) * (y1 - y2)) - ((y1 - y3) * (x1 - x2)))
            / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));

        // intersection only occurs if 0 <= t <= 1 & 0 <= u <= 1
        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            intersections += 1;
        }
    }

    intersections % 2 != 0
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

#[test]
#[cfg(test)]
fn inside_polygon_test_1() {
    let vector = Vec2::new(0.0, 0.0);
    let points = [
        Vec2::new(-1.0, -1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(-1.0, 1.0),
    ];

    let inside = inside_polygon(vector, &points);

    // Check resulting changes
    assert_eq!(inside, true);
}

#[test]
#[cfg(test)]
fn inside_polygon_test_2() {
    let vector = Vec2::new(1.1, 0.0);
    let points = [
        Vec2::new(-1.0, -1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(-1.0, 1.0),
    ];

    let inside = inside_polygon(vector, &points);

    // Check resulting changes
    assert_eq!(inside, false);
}

#[test]
#[cfg(test)]
fn inside_polygon_test_3() {
    let vector = Vec2::new(0.0, 0.5);
    let points = [
        Vec2::new(-1.0, -1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(-1.0, 1.0),
    ];

    let inside = inside_polygon(vector, &points);

    // Check resulting changes
    assert_eq!(inside, false);
}

#[test]
#[cfg(test)]
fn inside_polygon_test_4() {
    let vector = Vec2::new(0.0, 0.0);
    let points = [
        Vec2::new(-1.0, -1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 0.25),
        Vec2::new(-1.0, 1.0),
    ];

    let inside = inside_polygon(vector, &points);

    // Check resulting changes
    assert_eq!(inside, true);
}

#[test]
#[cfg(test)]
fn inside_polygon_test_5() {
    let vector = Vec2::new(0.0, 1.5);
    let points = [
        Vec2::new(-1.0, -1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(-1.0, 1.0),
    ];

    let inside = inside_polygon(vector, &points);

    // Check resulting changes
    assert_eq!(inside, false);
}
