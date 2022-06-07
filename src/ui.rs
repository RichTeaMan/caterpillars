use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::caterpillar::CaterpillarHead;

#[derive(Component)]
pub struct TextChanges;

#[derive(Component)]
pub struct NameUi;

#[derive(Component)]
pub struct UiInformation {
    pub active_caterpillar_name: String,
}

#[derive(Component)]
pub struct SelectedCaterpillar;

pub fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mono_font = asset_server.load("fonts/FiraMono-Regular.ttf");
    let regular_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: mono_font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(TextChanges);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(300.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: regular_font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(NameUi);
}

pub fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        text.sections[0].value = format!("{:.1} fps, {:.3} ms/frame", fps, frame_time * 1000.0,);
    }
}

pub fn update_flavour_text_system(
    mut query: Query<&mut Text, With<NameUi>>,
    selected_query: Query<&mut CaterpillarHead, With<SelectedCaterpillar>>,
) {
    for mut text in query.iter_mut() {
        let mut name: String = "".to_string();
        if !selected_query.is_empty() {
            name = format!("{:}", selected_query.single().name);
        }
        text.sections[0].value = name;
    }
}
