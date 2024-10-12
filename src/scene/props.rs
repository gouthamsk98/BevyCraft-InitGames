use bevy::{ ecs::entity, prelude::*, utils::info };
use bevy_mod_picking::prelude::*;
use crate::models::{ MeshType, MeshParameters, MeshId, CurrentMeshEntity };

pub fn spwan_prop(
    mut current_entity: ResMut<CurrentMeshEntity>,
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
    commands.spawn((
        PbrBundle {
            mesh: mesh,
            material: materials.add(params.color),
            transform: Transform::from_translation(params.position),
            ..default()
        },
        PickableBundle::default(),
        // On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        On::<Pointer<DragStart>>::run(set_current_entity),
        On::<Pointer<DragEnd>>::run(set_current_entity_to_none), // Re-enable picking
    ));
}
fn set_current_entity(
    event: Listener<Pointer<DragStart>>,
    mut current_entity: ResMut<CurrentMeshEntity>
) {
    let entity = event.target();
    current_entity.0 = Some(entity);
}
fn set_current_entity_to_none(mut current_entity: ResMut<CurrentMeshEntity>) {
    current_entity.0 = None;
}
pub fn spwan_gltf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Fox/Fox.glb")),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });
}
