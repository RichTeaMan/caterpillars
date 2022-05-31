use bevy::prelude::*;

#[derive(Component)]
pub struct Collider;

pub fn collision_system(
    meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Handle<Mesh>, &mut Collider)>,
) {
    for (handle, _collider) in query.iter_mut() {
        let mesh_option = meshes.get(handle.id);
        if let Some(mesh) = mesh_option {}

        println!("collide");
    }
}
