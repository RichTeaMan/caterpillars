mod camera;
mod caterpillar;
mod config;
mod foliage;
mod pick_events;
mod random;
mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_mod_picking::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.80, 0.92)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_startup_system(setup_scene)
        .add_startup_system(foliage::setup_foliage)
        .add_startup_system(caterpillar::setup_caterpillars)
        .add_startup_system(camera::spawn_camera)
        .add_startup_system(ui::infotext_system)
        .add_system(caterpillar::caterpillar_system)
        .add_system(camera::pan_orbit_camera)
        .add_system(ui::change_text_system)
        .add_system_to_stage(CoreStage::PostUpdate, pick_events::print_events)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_plane_handle = meshes.add(Mesh::from(shape::Plane {
        size: config::PLANE_SIZE,
    }));

    let ground_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.33, 0.49, 0.27),
        perceptual_roughness: 1.0,
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: ground_plane_handle,
        material: ground_material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // directional 'sun' light
    const HALF_SIZE: f32 = config::PLANE_SIZE / 2.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -100.0 * HALF_SIZE,
                far: 100.0 * HALF_SIZE,
                ..default()
            },
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
