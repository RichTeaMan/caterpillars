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

pub fn collision_system() {}

#[cfg(test)]
#[derive(Default, Component)]
struct CollisionTest {
    has_collided: bool,
}

#[test]
#[cfg(test)]
fn update_score_on_event() {
    // Setup app
    let mut app = App::new();

    //app.world.entity(entity).

    let callback: CollisionCallback =
        |commands, mut world, (collider_a, entity_a), (collider_b, entit_b)| {
            let option = world
                .query::<&mut CollisionTest>()
                .get_mut(&mut world, entity_a);
            if let Ok(mut ct) = option {
                ct.has_collided = true;
            }
        };

    app.world
        .spawn()
        .insert(Collider {
            collision_callback: callback.clone(),
        })
        .insert(CollisionTest::default());
    app.world
        .spawn()
        .insert(Collider {
            collision_callback: callback,
        })
        .insert(CollisionTest::default());

    // Add our systems
    app.add_system(collision_system);

    app.world.entities();

    // Run systems
    app.update();

    // Check resulting changes
    //assert_eq!(called_back, true);
}

#[test]
fn test_callback() {}
