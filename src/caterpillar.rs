use crate::{
    collision, config, dynamic_config::DynamicConfig, foliage::Food, random, toast::ToastEvent,
};
use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Component)]
pub struct CaterpillarHead {
    pub speed: f32,
    pub next: Option<Entity>,
    pub manually_controlled: bool,
    pub frames: i32,
    pub name: String,
    pub description: String,
}

#[derive(Component)]
pub struct CaterpillarPart {
    pub next: Option<Entity>,
}

pub fn caterpillar_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    dynamic_config: Res<DynamicConfig>,
    mut query: Query<(&mut Transform, &mut CaterpillarHead), Without<CaterpillarPart>>,
    mut part_query: Query<(&mut Transform, &mut CaterpillarPart), Without<CaterpillarHead>>,
) {
    for (mut transform, mut caterpillar) in query.iter_mut() {
        let direction;

        if caterpillar.manually_controlled {
            if keyboard_input.pressed(KeyCode::A) {
                transform.rotate(Quat::from_rotation_y(0.2));
            } else if keyboard_input.pressed(KeyCode::D) {
                transform.rotate(Quat::from_rotation_y(-0.2));
            }

            if keyboard_input.pressed(KeyCode::W) {
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

                caterpillar.frames = random::range_i32(10, 500);

                caterpillar.description = random::from_vec(&dynamic_config.thoughts);
            }
            direction = transform.forward();
            caterpillar.frames -= 1;
        }

        transform.translation += direction * caterpillar.speed * time.delta_seconds();

        let mut caterpillar_part = caterpillar.next;
        let mut parent_transform = transform.translation;

        while let Some(x) = caterpillar_part {
            if let Ok((mut part_transform, part)) = part_query.get_mut(x) {
                part_transform.look_at(parent_transform, Vec3::Y);

                let fwd = part_transform.forward();

                let distance = Vec3::distance(parent_transform, part_transform.translation);
                if distance > 3.0 {
                    part_transform.translation += fwd * caterpillar.speed * time.delta_seconds();
                }

                parent_transform = part_transform.translation;
                caterpillar_part = part.next;
            }
        }
    }
}

pub fn eat_check(
    mut commands: Commands,
    caterpillar_query: Query<(&mut Transform, &mut CaterpillarHead), Without<Food>>,
    food_query: Query<(Entity, &mut Transform, &mut Food)>,
    mut ev_toast: EventWriter<ToastEvent>,
) {
    for caterpillar in caterpillar_query.iter() {
        for food in food_query.iter() {
            if collision::collision_check(caterpillar.0.translation, food.1.translation, 4.0) {
                info!("{}: YUM YUM!!!", caterpillar.1.name);
                ev_toast.send(ToastEvent {
                    message: format!("{}: YUM YUM!!!", caterpillar.1.name),
                    expiry_tick: 5000,
                });
                commands.entity(food.0).despawn();
            }
        }
    }
}

pub fn setup_caterpillars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<DynamicConfig>,
) {
    let head_radius = 2.0;
    let body_radius = 1.5;
    let head_sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: head_radius,
        sectors: 16,
        stacks: 16,
    }));
    let eye_sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.25,
        sectors: 16,
        stacks: 16,
    }));
    let nose_sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.5,
        sectors: 16,
        stacks: 16,
    }));
    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: body_radius,
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

    let eye_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.0),
        ..default()
    });
    let nose_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 1.0),
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

    for _ in 0..config::STARTING_CATERPILLARS {
        let mut starting_vec = random::vec3(config::STARTING_CATERPILLAR_RADIUS);
        starting_vec.y = 3.0;
        let starting_transform = Transform::default().with_translation(starting_vec);

        let mut part_entity_option: Option<Entity> = Option::None;

        let length = random::range_i32(
            config::CATERPILLAR_MIN_LENGTH,
            config::CATERPILLAR_MAX_LENGTH,
        );
        for _ in 1..length {
            let caterpillar_part = CaterpillarPart {
                next: part_entity_option,
            };

            let part_entity = commands
                .spawn(PbrBundle {
                    mesh: sphere_handle.clone(),
                    material: sphere_material_handle.clone(),
                    transform: starting_transform,
                    ..default()
                })
                .insert(caterpillar_part)
                .with_children(|parent| {
                    // body spheres
                    parent.spawn(PbrBundle {
                        mesh: foot_sphere_handle.clone(),
                        material: foot_sphere_material_handle.clone(),
                        transform: Transform::from_xyz(3.5, -2.0, 0.0),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: foot_sphere_handle.clone(),
                        material: foot_sphere_material_handle.clone(),
                        transform: Transform::from_xyz(-3.5, -2.0, 0.0),
                        ..default()
                    });
                })
                .id();

            part_entity_option = Option::from(part_entity);
        }

        // head sphere
        commands
            .spawn(PbrBundle {
                mesh: head_sphere_handle.clone(),
                material: head_material_handle.clone(),
                transform: starting_transform,
                ..default()
            })
            .insert(PickableBundle::default())
            .insert(CaterpillarHead {
                speed: random::range_f32(
                    config::CATERPILLAR_MIN_SPEED,
                    config::CATERPILLAR_MAX_SPEED,
                ),
                next: part_entity_option,
                manually_controlled: false,
                frames: 0,
                name: random::from_vec(&config.names),
                description: random::from_vec(&config.thoughts),
            })
            .with_children(|parent| {
                // nose
                parent.spawn(PbrBundle {
                    mesh: nose_sphere_handle.clone(),
                    material: nose_material_handle.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -head_radius),
                    ..default()
                });
                let eye_offset_x = (head_radius / 2.0) - 0.3;
                let eye_offset_y = head_radius / 2.0;
                // left eye
                parent.spawn(PbrBundle {
                    mesh: eye_sphere_handle.clone(),
                    material: eye_material_handle.clone(),
                    transform: Transform::from_xyz(
                        eye_offset_x,
                        eye_offset_y,
                        -(head_radius - 0.5),
                    ),
                    ..default()
                });
                // right eye
                parent.spawn(PbrBundle {
                    mesh: eye_sphere_handle.clone(),
                    material: eye_material_handle.clone(),
                    transform: Transform::from_xyz(
                        -eye_offset_x,
                        eye_offset_y,
                        -(head_radius - 0.5),
                    ),
                    ..default()
                });
            });
    }
}
