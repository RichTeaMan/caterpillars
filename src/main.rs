//! Illustrates how to create parent-child relationships between entities and how parent transforms
//! are propagated to their descendants.

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_caterpillars)
        .add_system(caterpillar_system)
        .run();
}

#[derive(Component)]
struct CaterpillarHead {
    pub parts: Vec<i32>,
    pub speed: f32,
    pub next: Option<CaterpillarPart>,
}

#[derive(Component)]
struct CaterpillarPart {
    pub next: Box<CaterpillarPart>,
}

fn caterpillar_system(keyboard_input: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<(&mut Transform, &mut CaterpillarHead)>) {
    
    for (mut transform, mut caterpillar) in query.iter_mut() {
        //transform.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds());
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        if keyboard_input.pressed(KeyCode::A) {
            direction = transform.left();
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction = transform.right();
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction = transform.forward();
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction = transform.back();
        }
        
        transform.translation += direction * caterpillar.speed * time.delta_seconds();
    }
}

fn setup_caterpillars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 2.0,
        sectors: 16,
        stacks: 16,
    }));
    let sphere_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    // parent sphere
    commands
        .spawn_bundle(PbrBundle {
            mesh: sphere_handle.clone(),
            material: sphere_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(CaterpillarHead {
            parts: vec![1, 0, 3],
            speed: 1.5,
            next: Option::None,
        });
        /*.with_children(|parent| {
            // child cube
            parent.spawn_bundle(PbrBundle {
                mesh: sphere_handle.clone(),
                material: sphere_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 4.0),
                ..default()
            });
        })
        .with_children(|parent| {
            // child cube
            parent.spawn_bundle(PbrBundle {
                mesh: sphere_handle.clone(),
                material: sphere_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 7.0),
                ..default()
            });
        })
        .with_children(|parent| {
            // child cube
            parent.spawn_bundle(PbrBundle {
                mesh: sphere_handle,
                material: sphere_material_handle,
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            });
        });
        */
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
