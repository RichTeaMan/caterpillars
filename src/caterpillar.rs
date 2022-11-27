use std::{f32::consts::PI, time::Duration};

use crate::{
    camera::FollowCamera, collision, dynamic_config::DynamicConfig, foliage::Food, random,
    toast::ToastEvent, ui::SelectedCaterpillar,
};
use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    *,
};

#[derive(PartialEq, Eq)]
pub enum AngleOffsetDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct CaterpillarHead {
    pub speed: f32,
    pub next: Option<Entity>,
    pub manually_controlled: bool,
    pub frames: i32,
    pub name: String,
    pub description: String,

    pub angle: f32,
    pub angle_offset: f32,
    pub angle_offset_direction: AngleOffsetDirection,
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
                caterpillar.angle_offset = 0.0;
                let mut angle = rand::random();
                angle *= 2.0 * std::f32::consts::PI;
                caterpillar.angle = angle;
                transform.rotate(Quat::from_rotation_y(angle));

                caterpillar.frames = random::range_i32(48000, 50000);

                caterpillar.description = random::from_vec(&dynamic_config.thoughts);
            }
            direction = transform.forward();
            caterpillar.frames -= 1;
        }

        const ANGLE_MAX: f32 = 1.6;
        const ANGLE_CHANGE: f32 = 0.001;
        if caterpillar.angle_offset.abs() > ANGLE_MAX {
            if caterpillar.angle_offset_direction == AngleOffsetDirection::Left {
                caterpillar.angle_offset_direction = AngleOffsetDirection::Right;
            } else {
                caterpillar.angle_offset_direction = AngleOffsetDirection::Left;
            }
        }
        let dyn_angle_change = (1.0
            - (caterpillar.angle_offset.abs() - (ANGLE_MAX / 2.0))
                .abs()
                .asin())
            * 0.01
            + ANGLE_CHANGE;
        if caterpillar.angle_offset_direction == AngleOffsetDirection::Left {
            caterpillar.angle_offset = caterpillar.angle_offset - dyn_angle_change;
        } else {
            caterpillar.angle_offset = caterpillar.angle_offset + dyn_angle_change;
        }

        transform.rotation = Quat::from_rotation_y(caterpillar.angle + caterpillar.angle_offset);

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
    let leg_length = 3.0;
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
    let leg_mesh_handle = meshes.add(Mesh::from(shape::Capsule {
        radius: 0.5,
        //rings: 4,
        depth: leg_length - 0.5,
        //latitudes: 8,
        //longitudes: 8,
        //uv_profile: shape::CapsuleUvProfile::Aspect,
        ..default()
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

    for _ in 0..config.starting_caterpillars {
        let mut starting_vec = random::vec3(config.starting_caterpillar_radius);
        starting_vec.y = 3.0;
        let starting_transform = Transform::default().with_translation(starting_vec);

        let mut part_entity_option: Option<Entity> = Option::None;

        let length =
            random::range_i32(config.caterpillar_min_length, config.caterpillar_max_length);
        for _ in 1..length {
            let caterpillar_part = CaterpillarPart {
                next: part_entity_option,
            };

            // right leg tween
            let leg_tween_r = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(750),
                TransformRotationLens {
                    start: Quat::from_euler(EulerRot::XYZ, 0.0, -0.5, 0.5 * PI),
                    end: Quat::from_euler(EulerRot::XYZ, 0.0, 0.5, 0.5 * PI),
                },
            )
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
            .with_repeat_count(RepeatCount::Infinite);

            // foot tween
            let foot_tween_l = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(750),
                TransformPositionLens {
                    start: Vec3::new(-3.5, -2.0, -1.5),
                    end: Vec3::new(-3.5, -2.0, 1.5),
                },
            )
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
            .with_repeat_count(RepeatCount::Infinite);

            // foot tween
            let foot_tween_r = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: Vec3::new(3.5, -2.0, -1.5),
                    end: Vec3::new(3.5, -2.0, 1.5),
                },
            )
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
            .with_repeat_count(RepeatCount::Infinite);

            let part_entity = commands
                // body sphere
                .spawn(PbrBundle {
                    mesh: sphere_handle.clone(),
                    material: sphere_material_handle.clone(),
                    transform: starting_transform,
                    ..default()
                })
                .insert(caterpillar_part)
                .with_children(|parent| {
                    // right leg
                    parent
                        .spawn(SpatialBundle {
                            transform: starting_transform.with_rotation(Quat::from_euler(
                                EulerRot::XYZ,
                                0.0,
                                0.5,
                                0.5 * PI,
                            )),
                            ..default()
                        })
                        .insert(Animator::new(leg_tween_r))
                        .with_children(|parent| {
                            parent.spawn(PbrBundle {
                                mesh: leg_mesh_handle.clone(),
                                material: sphere_material_handle.clone(),
                                transform: Transform::IDENTITY
                                    // here X is the vertical height of the leg joint. Set to leg_length because the leg model
                                    // is aligned upwards so the rotation to horizontal means it's too high.
                                    .with_translation(Vec3::new(-leg_length, -leg_length, 0.0)),
                                ..default()
                            });
                        });

                    // feet spheres
                    parent
                        .spawn(PbrBundle {
                            mesh: foot_sphere_handle.clone(),
                            material: foot_sphere_material_handle.clone(),
                            transform: Transform::from_xyz(-3.5, -2.0, 0.0),
                            ..default()
                        })
                        .insert(Animator::new(foot_tween_l));
                    parent
                        .spawn(PbrBundle {
                            mesh: foot_sphere_handle.clone(),
                            material: foot_sphere_material_handle.clone(),
                            transform: Transform::from_xyz(3.5, -2.0, 0.0),
                            ..default()
                        })
                        .insert(Animator::new(foot_tween_r));
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
                    config.caterpillar_min_speed,
                    config.caterpillar_max_speed,
                ),
                next: part_entity_option,
                manually_controlled: false,
                frames: 0,
                name: random::from_vec(&config.names),
                description: random::from_vec(&config.thoughts),
                angle: 0.0,
                angle_offset: 0.0,
                angle_offset_direction: AngleOffsetDirection::Left,
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

pub fn camera_follow_caterpillar_system(
    mut follow_camera: ResMut<FollowCamera>,
    selected_caterpillar_query: Query<(&SelectedCaterpillar, &Transform)>,
) {
    for (_, transform) in selected_caterpillar_query.iter() {
        follow_camera.translation = transform.translation;
        follow_camera.enabled = true;
    }
}
