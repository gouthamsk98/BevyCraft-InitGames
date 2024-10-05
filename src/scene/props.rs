use bevy::prelude::*;
use crate::models::{ MeshType, MeshParameters };
pub fn spwan_prop(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    params: MeshParameters
) {
    let mesh = match params.dimensions {
        MeshType::Cube { width, height, depth } => meshes.add(Cuboid::new(width, height, depth)),
        MeshType::Sphere { radius } => meshes.add(Sphere::new(radius)),
        MeshType::Cylinder { radius, height } => meshes.add(Cylinder::new(radius, height)),
        MeshType::Capsule3D { radius, height } => meshes.add(Capsule3d::new(radius, height)),
        MeshType::Plane3D { width, height } =>
            meshes.add(Plane3d::default().mesh().size(width, height)),
    };
    commands.spawn(PbrBundle {
        mesh: mesh,
        material: materials.add(params.color),
        transform: Transform::from_translation(params.position),
        ..default()
    });
}
