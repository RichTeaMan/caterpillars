use serde::{Deserialize, Serialize};
use serde_json::Result;

use bevy::prelude::*;

#[derive(Component, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DynamicConfig {
    pub names: Vec<String>,
    pub thoughts: Vec<String>,
    pub child_thoughts: Vec<String>,
}

pub fn create_dynamic_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let data_handle: HandleUntyped = asset_server.load_untyped("data.json");

    let asset_path = asset_server.get_handle_path(data_handle).unwrap();

    let a = asset_server.asset_io().clone();

    let load_task = async move { a.load_path(asset_path.path()).await };

    let load_result = futures::executor::block_on(load_task);

    let json_string;
    match load_result {
        Ok(result) => {
            json_string = String::from_utf8_lossy(&result).into_owned();
        }
        Err(e) => panic!("{e}"),
    }

    let dynamic_config_result: Result<DynamicConfig> = serde_json::from_str(json_string.as_str());

    match dynamic_config_result {
        Ok(dynamic_config) => {
            commands.spawn().insert(dynamic_config);
        }
        Err(e) => panic!("Error loading dynamic config {e}."),
    }
}
