use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct ToastEvent {
    pub message: String,
    pub expiry_tick: u128,
}

pub fn toast_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_levelup: EventReader<ToastEvent>,
    mut toast_query: Query<(Entity, &mut ToastEvent, &mut Style)>,
    time: Res<Time>,
) {
    let milliseconds = time.time_since_startup().as_millis();
    let mut y_position = 50.0;
    let y_margin = 30.0;
    for mut toast_bundle in toast_query.iter_mut() {
        if toast_bundle.1.expiry_tick < milliseconds {
            commands.entity(toast_bundle.0).despawn();
        } else {
            toast_bundle.2.position.top = Val::Px(y_position);
            y_position = y_position + y_margin;
        }
    }

    let regular_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    for ev in ev_levelup.iter() {
        let mut toast_event = ev.clone();
        toast_event.expiry_tick = toast_event.expiry_tick + milliseconds;
        info!("Toast: {}", ev.message);
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(y_position),
                        right: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: ev.message.to_string(),
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
            .insert(toast_event);
        y_position = y_position + y_margin;

        /*
        match toast_manager.toast_events.add(ev.clone()) {
            Err(r) => warn!("Failed to add toast event: {}.", r),
            _ => (),
        }
        */
    }
}
