use bevy::prelude::*;

use crate::collision::Collider;

#[derive(Component)]
pub struct Wall {
    pub p1: Vec2,
    pub p2: Vec2,
}

pub fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.44, 0.33, 0.23),
        ..default()
    });

    let walls = vec![Wall {
        p1: Vec2::new(0.0, 0.0),
        p2: Vec2::new(10.0, 10.0),
    }];

    let wall_plane_handle = meshes.add(Mesh::from(shape::Box::new(2.0, 10.0, 200.0)));

    
    commands
        .spawn_bundle(PbrBundle {
            mesh: wall_plane_handle,
            material: wall_material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        })
        ;//.insert(Wall);
}
