use bevy::prelude::*;

use crate::{dynamic_config::DynamicConfig, random};

#[derive(Component)]
pub struct Food;

#[derive(Resource)]
pub struct FoliageAssets {
    pub tree_box_handle: Handle<Mesh>,
    pub tree_trunk_material_handle: Handle<StandardMaterial>,
    pub tree_green_mesh_handle: Handle<Mesh>,
    pub tree_green_material_handle: Handle<StandardMaterial>,

    pub bush_handle: Handle<Mesh>,
    pub bush_material_handle: Handle<StandardMaterial>,
}

pub fn setup_foliage_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<DynamicConfig>,
) {
    let tree_box_handle = meshes.add(Mesh::from(shape::Box::new(2.0, config.tree_height, 2.0)));
    let tree_trunk_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.38, 0.23, 0.08),
        perceptual_roughness: 1.0,
        ..default()
    });
    let tree_green_mesh_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 8.0,
        stacks: 16,
        sectors: 16,
    }));
    let tree_green_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.28, 0.37, 0.04),
        perceptual_roughness: 1.0,
        ..default()
    });

    let bush_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 4.0,
        stacks: 16,
        sectors: 16,
    }));
    let bush_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.25, 0.0),
        perceptual_roughness: 1.0,
        ..default()
    });

    let foliage_assets = FoliageAssets {
        tree_box_handle,
        tree_trunk_material_handle,
        tree_green_mesh_handle,
        tree_green_material_handle,
        bush_handle,
        bush_material_handle,
    };
    commands.insert_resource(foliage_assets);
}

pub fn setup_foliage(
    mut commands: Commands,
    foliage_assets: Res<FoliageAssets>,
    config: Res<DynamicConfig>,
) {
    for _ in 0..config.starting_trees {
        commands = create_tree_prv(
            commands,
            &foliage_assets,
            config.plane_size,
            config.tree_height,
        );
    }

    for _ in 0..config.starting_bushes {
        commands = create_bush_prv(commands, &foliage_assets, config.plane_size);
    }
}

fn create_tree_prv<'w, 's>(
    mut commands: Commands<'w, 's>,
    foliage_assets: &Res<FoliageAssets>,
    plane_size: f32,
    tree_height: f32,
) -> Commands<'w, 's> {
    let mut starting_vec = random::vec3(plane_size / 2.0);
    starting_vec.y = tree_height / 2.0;
    let starting_transform = Transform::default().with_translation(starting_vec);

    let mut tree_sphere_tranform_1 = Transform::from_xyz(0.0, tree_height / 2.0, 0.0);
    tree_sphere_tranform_1.scale = Vec3::new(1.0, 1.0, 1.0);

    let mut tree_sphere_tranform_2 = Transform::from_xyz(0.0, (tree_height / 2.0) - 20.0, 0.0);
    tree_sphere_tranform_2.scale = Vec3::new(2.0, 2.0, 2.0);

    let mut tree_sphere_tranform_3 = Transform::from_xyz(0.0, (tree_height / 2.0) - 48.0, 0.0);
    tree_sphere_tranform_3.scale = Vec3::new(3.0, 3.0, 3.0);

    commands
        .spawn(PbrBundle {
            mesh: foliage_assets.tree_box_handle.clone(),
            material: foliage_assets.tree_trunk_material_handle.clone(),
            transform: starting_transform,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: foliage_assets.tree_green_mesh_handle.clone(),
                material: foliage_assets.tree_green_material_handle.clone(),
                transform: tree_sphere_tranform_1,
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: foliage_assets.tree_green_mesh_handle.clone(),
                material: foliage_assets.tree_green_material_handle.clone(),
                transform: tree_sphere_tranform_2,
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: foliage_assets.tree_green_mesh_handle.clone(),
                material: foliage_assets.tree_green_material_handle.clone(),
                transform: tree_sphere_tranform_3,
                ..default()
            });
        });
    commands
}

#[allow(dead_code)]
pub fn create_tree(
    commands: Commands,
    foliage_assets: Res<FoliageAssets>,
    config: Res<DynamicConfig>,
) {
    create_tree_prv(
        commands,
        &foliage_assets,
        config.plane_size,
        config.tree_height,
    );
}

fn create_bush_prv<'w, 's>(
    mut commands: Commands<'w, 's>,
    foliage_assets: &Res<FoliageAssets>,
    plane_size: f32,
) -> Commands<'w, 's> {
    let mut starting_vec = random::vec3(plane_size / 2.0);
    starting_vec.y = 4.0 / 2.0;
    let starting_transform = Transform::default().with_translation(starting_vec);

    commands
        .spawn(PbrBundle {
            mesh: foliage_assets.bush_handle.clone(),
            material: foliage_assets.bush_material_handle.clone(),
            transform: starting_transform,
            ..default()
        })
        .insert(Food);
    commands
}

pub fn spawn_bushes(
    mut commands: Commands,
    foliage_assets: Res<FoliageAssets>,
    config: Res<DynamicConfig>,
) {
    for _ in 0..config.bush_spawn_rate {
        commands = create_bush_prv(commands, &foliage_assets, config.plane_size);
    }
}
