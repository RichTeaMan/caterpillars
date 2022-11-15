use std::env;

use bevy::prelude::*;

use crate::AppState;

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Resource)]
#[uuid = "3b661374-e6a2-11ec-8fea-0242ac120002"]
#[serde(rename_all = "camelCase")]
pub struct DynamicConfig {
    pub plane_size: f32,
    pub starting_caterpillars: i32,

    pub starting_caterpillar_radius: f32,

    pub starting_bushes: i32,

    pub starting_trees: i32,

    pub bush_spawn_rate: i32,

    pub tree_height: f32,

    /** Minimum length of a caterpillar. */
    pub caterpillar_min_length: i32,

    /** Maximum length of a caterpillar. */
    pub caterpillar_max_length: i32,

    /** Minimum speed of a caterpillar. */
    pub caterpillar_min_speed: f32,

    /** Maximum speed of a caterpillar. */
    pub caterpillar_max_speed: f32,

    pub enable_shadows: bool,

    pub names: Vec<String>,
    pub thoughts: Vec<String>,
    pub child_thoughts: Vec<String>,
}

#[derive(Resource)]
pub struct DynamicConfigHandleHolder(Handle<DynamicConfig>);

pub fn create_dynamic_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<State<AppState>>,
) {
    let mut data_file: String = "data.json".to_string();
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg.ends_with(".json") {
            data_file = arg.clone();
        }
    }
    info!("Using data from {}.", data_file);

    let data_handle: Handle<DynamicConfig> = asset_server.load(data_file.as_str());
    commands.insert_resource(DynamicConfigHandleHolder(data_handle));
    app_state.set(AppState::ConfigLoad).unwrap();
    info!("Config created.");
}

pub fn load_dynamic_config(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    handle: Res<DynamicConfigHandleHolder>,
    mut dynamic_config_assets: ResMut<Assets<DynamicConfig>>,
) {
    if let Some(dynamic_config) = dynamic_config_assets.remove(handle.0.id()) {
        commands.insert_resource(dynamic_config);
        app_state.set(AppState::PreLoad).unwrap();
        info!("Config loaded.");
    } else {
        // this error appers to happen once. further investigation required.
        error!("Dynamic config could not be loaded.");
    }
}
