mod camera;
mod caterpillar;
mod collision;
mod config;
mod random;
mod walls;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.53, 0.80, 0.92)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_startup_system(walls::setup_walls)
        .add_startup_system(caterpillar::setup_caterpillars)
        .add_startup_system(camera::spawn_camera)
        .add_system(caterpillar::caterpillar_system)
        .add_system(camera::pan_orbit_camera)
        .add_system(collision::collision_system)
        .run();
}

fn setup_scene(
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
