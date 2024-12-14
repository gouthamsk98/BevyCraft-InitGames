//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevycraft::{ scene, models::CurrentMeshEntity, interaction };
use bevy_mod_picking::DefaultPickingPlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some(String::from("#main-canvas")),
                    fit_canvas_to_parent: true, // Ensures the canvas fits the parent container
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            interaction::camera_controller::CameraControllerPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(CurrentMeshEntity::default())
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            scene::plane::add_grid,
            interaction::mouse::handle_element_interaction,
        ))
        .add_systems(Update, (
            interaction::mouse::mouse_input_system,
            interaction::keyboard::keyboard_input_system,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    scene::camera::spawn_camera(&mut commands);
    scene::plane::spwan_plane(&mut commands, &mut meshes, &mut materials);
    scene::light::spwan_light(&mut commands);
    commands
        .spawn((
            SceneBundle {
                scene: asset_server.load(
                    GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")
                ),

                ..default()
            },
            RigidBody::Dynamic,
        ))
        .insert(Collider::capsule(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(Restitution::coefficient(0.7));
}
