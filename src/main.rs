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
    pub speed: f32,
    pub next: Option<Entity>,
}

#[derive(Component)]
struct CaterpillarPart {
    pub next: Option<Entity>,
}

fn caterpillar_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CaterpillarHead), Without<CaterpillarPart>>,
    mut part_query: Query<(&mut Transform, &mut CaterpillarPart), Without<CaterpillarHead>>,
) {
    for (mut transform, caterpillar) in query.iter_mut() {
        let direction;
        if keyboard_input.pressed(KeyCode::A) {
            direction = transform.left();
        } else if keyboard_input.pressed(KeyCode::D) {
            direction = transform.right();
        } else if keyboard_input.pressed(KeyCode::W) {
            direction = transform.forward();
        } else if keyboard_input.pressed(KeyCode::S) {
            direction = transform.back();
        } else {
            continue;
        }

        transform.translation += direction * caterpillar.speed * time.delta_seconds();

        let mut caterpillar_part = caterpillar.next;
        let mut parent_transform = transform.translation;

        loop {
            match caterpillar_part {
                Some(x) => {
                    if let Ok((mut part_transform, part)) = part_query.get_mut(x) {
                        let _ = part_transform.look_at(parent_transform, Vec3::Y);

                        let fwd = part_transform.forward();

                        let distance = Vec3::distance(parent_transform, part_transform.translation);
                        if distance > 3.0 {
                            part_transform.translation +=
                                fwd * caterpillar.speed * time.delta_seconds();
                        }

                        parent_transform = part_transform.translation;
                        caterpillar_part = part.next;
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
}

fn setup_caterpillars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let head_sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 2.0,
        sectors: 16,
        stacks: 16,
    }));
    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 1.5,
        sectors: 16,
        stacks: 16,
    }));
    let head_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        ..default()
    });
    let sphere_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    let mut part_entity_option: Option<Entity> = Option::None;
    for _ in 1..5 {
        let caterpillar_part = CaterpillarPart {
            next: part_entity_option,
        };

        let part_entity = commands
            .spawn_bundle(PbrBundle {
                mesh: sphere_handle.clone(),
                material: sphere_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            })
            .insert(caterpillar_part)
            .id()
            .clone();

        part_entity_option = Option::from(part_entity);
    }

    // parent sphere
    commands
        .spawn_bundle(PbrBundle {
            mesh: head_sphere_handle.clone(),
            material: head_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(CaterpillarHead {
            speed: 1.5,
            next: part_entity_option,
        });
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
