use bevy::prelude::*;

use crate::AppState;

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Resource)]
#[uuid = "3b661374-e6a2-11ec-8fea-0242ac120002"]
#[serde(rename_all = "camelCase")]
pub struct DynamicConfig {
    pub names: Vec<String>,
    pub thoughts: Vec<String>,
    pub child_thoughts: Vec<String>,
}

#[derive(Resource)]
pub struct DynamicConfigHandleHolder(Handle<DynamicConfig>);

pub fn create_dynamic_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let data_handle: Handle<DynamicConfig> = asset_server.load("data.json");
    commands.insert_resource(DynamicConfigHandleHolder(data_handle));
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
        app_state.set(AppState::Loading).unwrap();
        info!("Config loaded.");
    } else {
        // this error appers to happen once. further investigation required.
        error!("Dynamic config could not be loaded.");
    }
}
