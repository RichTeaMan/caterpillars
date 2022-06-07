use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::{caterpillar::CaterpillarHead, ui::SelectedCaterpillar};

pub fn print_events(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    selected_query: Query<(&mut SelectedCaterpillar, &mut CaterpillarHead, Entity)>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("Mesh Selection {:?}", e),
            PickingEvent::Hover(e) => info!("Mesh Hover {:?}", e),
            PickingEvent::Clicked(e) => {
                for (_, _, selected_entity) in selected_query.iter() {
                    commands
                        .entity(selected_entity)
                        .remove::<SelectedCaterpillar>();
                }
                commands.entity(e.to_owned()).insert(SelectedCaterpillar);
                info!("Mesh clicked {:?}", e);
            }
        }
    }
}
