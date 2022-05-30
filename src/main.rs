mod camera;

use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.53, 0.80, 0.92)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_caterpillars)
        .add_startup_system(camera::spawn_camera)
        .add_system(caterpillar_system)
        .add_system(camera::pan_orbit_camera)
        .run();
}

#[derive(Component)]
struct CaterpillarHead {
    pub speed: f32,
    pub next: Option<Entity>,
    pub manually_controlled: bool,
    pub frames: i32,
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
    for (mut transform, mut caterpillar) in query.iter_mut() {
        let direction;

        if caterpillar.manually_controlled {
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
        } else {
            if caterpillar.frames == 0 {
                let mut angle = rand::random();
                angle *= 2.0 * std::f32::consts::PI;
                transform.rotate(Quat::from_rotation_y(angle));

                caterpillar.frames = rand::thread_rng().gen_range(10..500);
            }
            direction = transform.forward();
            caterpillar.frames = caterpillar.frames - 1;
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
    let ground_plane_handle = meshes.add(Mesh::from(shape::Plane { size: 5000.0 }));

    let ground_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.33, 0.49, 0.27),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: ground_plane_handle,
        material: ground_material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

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
    let foot_sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.25,
        sectors: 6,
        stacks: 6,
    }));
    let head_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        ..default()
    });
    let sphere_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });
    let foot_sphere_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.0),
        ..default()
    });

    for _ in 1..20 {
        let mut part_entity_option: Option<Entity> = Option::None;

        let length = rand::thread_rng().gen_range(3..20);
        for _ in 1..length {
            let caterpillar_part = CaterpillarPart {
                next: part_entity_option,
            };

            let part_entity = commands
                .spawn_bundle(PbrBundle {
                    mesh: sphere_handle.clone(),
                    material: sphere_material_handle.clone(),
                    transform: Transform::from_xyz(0.0, 3.0, 0.0),
                    ..default()
                })
                .insert(caterpillar_part)
                .with_children(|parent| {
                    // child cube
                    parent.spawn_bundle(PbrBundle {
                        mesh: foot_sphere_handle.clone(),
                        material: foot_sphere_material_handle.clone(),
                        transform: Transform::from_xyz(3.5, -2.0, 0.0),
                        ..default()
                    });
                    parent.spawn_bundle(PbrBundle {
                        mesh: foot_sphere_handle.clone(),
                        material: foot_sphere_material_handle.clone(),
                        transform: Transform::from_xyz(-3.5, -2.0, 0.0),
                        ..default()
                    });
                })
                .id()
                .clone();

            part_entity_option = Option::from(part_entity);
        }

        // parent sphere
        commands
            .spawn_bundle(PbrBundle {
                mesh: head_sphere_handle.clone(),
                material: head_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 3.0, 0.0),
                ..default()
            })
            .insert(CaterpillarHead {
                speed: 1.5,
                next: part_entity_option,
                manually_controlled: false,
                frames: 0,
            });
    }
    // directional 'sun' light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}
