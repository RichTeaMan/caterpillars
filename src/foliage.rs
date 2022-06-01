use bevy::prelude::*;

use crate::{config, random};

pub fn setup_foliage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tree_height = 100.0;
    let tree_box_handle = meshes.add(Mesh::from(shape::Box::new(2.0, tree_height, 2.0)));
    let tree_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.38, 0.23, 0.08),
        perceptual_roughness: 1.0,
        ..default()
    });
    for _ in 0..config::STARTING_FOLIAGE {
        let mut starting_vec = random::vec3(config::PLANE_SIZE / 2.0);
        starting_vec.y = tree_height / 2.0;
        let starting_transform = Transform::default().with_translation(starting_vec);

        commands.spawn_bundle(PbrBundle {
            mesh: tree_box_handle.clone(),
            material: tree_material_handle.clone(),
            transform: starting_transform,
            ..default()
        });
    }
}
