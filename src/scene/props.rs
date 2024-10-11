use bevy::{ prelude::*, utils::info };
use bevy_mod_picking::prelude::*;
use crate::models::{ MeshType, MeshParameters, MeshId };

pub fn spwan_prop(
    point: Vec3,
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
    commands
        .spawn((
            PbrBundle {
                mesh: mesh,
                material: materials.add(params.color),
                transform: Transform::from_translation(params.position),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            On::<Pointer<Drag>>::target_component_mut::<Transform>(move |drag, transform| {
                todo!()
            }),
            On::<Pointer<Click>>::run(on_mesh_click),
        ))
        .insert(MeshId(1));
}
fn on_mesh_click(_click: &Pointer<Click>, _mesh_id: &MeshId) {
    info!("Mesh clicked!");
}
