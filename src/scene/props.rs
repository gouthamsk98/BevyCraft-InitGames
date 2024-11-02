use bevy::{ ecs::entity, prelude::*, utils::info };
use bevy_mod_picking::prelude::*;
use bevy::scene::prelude::*;
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
/// Helper resource for tracking our asset
#[derive(Resource)]
struct MyAssetPack(Handle<Gltf>);
pub fn spwan_gltf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(SceneBundle {
        // This is equivalent to "models/FlightHelmet/FlightHelmet.gltf#Scene0"
        // The `#Scene0` label here is very important because it tells bevy to load the first scene in the glTF file.
        // If this isn't specified bevy doesn't know which part of the glTF file to load.
        scene: asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")
        ),
        // You can use the transform to give it a position
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
        ..Default::default()
    });
}
