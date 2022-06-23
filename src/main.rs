mod camera;
mod caterpillar;
mod collision;
mod config;
mod dynamic_config;
mod foliage;
mod pick_events;
mod random;
mod ui;

use std::cell::RefCell;

use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_mod_picking::*;
use dynamic_config::DynamicConfig;
use wasm_bindgen::prelude::*;

const TIMESTEP_1_PER_SECOND: f64 = 1.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    PreLoad,
    Loading,
    Level,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.80, 0.92)))
        .insert_resource(WindowDescriptor {
            width: config::START_RESOLUTION_WIDTH,
            height: config::START_RESOLUTION_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(JsonAssetPlugin::<DynamicConfig>::new(&["json"]))
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system_to_stage(
            StartupStage::PreStartup,
            dynamic_config::create_dynamic_config,
        )
        .add_startup_system_to_stage(StartupStage::PreStartup, foliage::setup_foliage_assets)
        .add_startup_system_to_stage(StartupStage::Startup, dynamic_config::load_dynamic_config)
        .add_state(AppState::PreLoad)
        .add_system_set(
            SystemSet::on_update(AppState::PreLoad)
                .with_system(dynamic_config::load_dynamic_config),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(setup_scene)
                .with_system(foliage::setup_foliage)
                .with_system(caterpillar::setup_caterpillars)
                .with_system(camera::spawn_camera)
                .with_system(ui::infotext_system)
                .with_system(loading_completed),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Level)
                .with_system(caterpillar::caterpillar_system)
                .with_system(camera::pan_orbit_camera)
                .with_system(ui::change_text_system)
                .with_system(ui::update_flavour_text_system)
                .with_system(ui::update_debug_ui_system)
                .with_system(pick_events::print_events)
                .with_system(window_resize_system)
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(foliage::create_bush),
        )
        .run();
}

thread_local!(static GLOBAL_X: RefCell<i32>  = RefCell::new(0));
thread_local!(static GLOBAL_Y: RefCell<i32>  = RefCell::new(0));
thread_local!(static GLOBAL_SCALE: RefCell<f64>  = RefCell::new(0.0));

#[wasm_bindgen]
pub fn caterpilar_game_resize(width: i32, height: i32) {
    GLOBAL_X.with(|text| *text.borrow_mut() = width);
    GLOBAL_Y.with(|text| *text.borrow_mut() = height);
}

#[wasm_bindgen]
pub fn caterpilar_game_resize_with_scale(width: i32, height: i32, scale: f64) {
    GLOBAL_X.with(|text| *text.borrow_mut() = width);
    GLOBAL_Y.with(|text| *text.borrow_mut() = height);
    GLOBAL_SCALE.with(|text| *text.borrow_mut() = scale);
}

fn window_resize_system(mut windows: ResMut<Windows>, keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::R) {
        let window = windows.get_primary_mut().unwrap();
        info!("Window size was: {},{}", window.width(), window.height());
        window.set_resolution(800.0, 600.0);
    }

    let mut x = 0;
    let mut y = 0;
    let mut scale = 0.0;
    GLOBAL_X.with(|text| x = *text.borrow());
    GLOBAL_Y.with(|text| y = *text.borrow());
    GLOBAL_SCALE.with(|text| scale = *text.borrow());

    if x != 0 && y != 0 {
        let window = windows.get_primary_mut().unwrap();
        info!("Window size was: {},{}", window.width(), window.height());
        window.set_resolution(x as f32, y as f32);
        info!("Window size now: {},{}", x, y);

        if scale != 0.0 {
            info!(
                "Window scale was: {:?} {:?}",
                window.scale_factor(),
                window.scale_factor_override()
            );
            //window.set_scale_factor_override(Some(4.0));
            window.set_scale_factor_override(Some(scale));
            info!("Window scale now: {}", scale);
        }
        GLOBAL_X.with(|text| *text.borrow_mut() = 0);
        GLOBAL_Y.with(|text| *text.borrow_mut() = 0);
        GLOBAL_SCALE.with(|text| *text.borrow_mut() = 0.0);
    }
}

fn loading_completed(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::Level).unwrap();
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
            shadows_enabled: config::ENABLE_SHADOWS,
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
