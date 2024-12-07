//! A simple 3D scene with light shining over a cube sitting on a plane.
use wasm_bindgen::prelude::*;
use bevy::prelude::*;
use bevycraft::{ scene, models::{ MeshType, MeshParameters, CurrentMeshEntity } };
use bevy_mod_picking::DefaultPickingPlugins;
const FOX_PATH: &str = "models/animated/Fox.glb";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some(String::from("#main-canvas")),
                    ..default()
                }),
                ..default()
            }),
            // interaction::camera_controller::CameraControllerPlugin,
        ))

        .insert_resource(CurrentMeshEntity::default())
        // .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (scene::plane::add_grid, scene::plane::handle_element_interaction))
        // .add_systems(Update, (
        //     interaction::mouse::mouse_input_system,
        //     interaction::keyboard::keyboard_input_system,
        // ))
        .run();
}

/// set up a simple 3D scene
use bevy::{ prelude::*, render::camera::ScalingMode };
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    scene::camera::spawn_camera(&mut commands);
    scene::plane::spwan_plane(&mut commands, &mut meshes, &mut materials);
    scene::light::spwan_light(&mut commands);
    scene::props::spwan_gltf(commands, meshes, materials, asset_server);
    // commands.spawn(
    //     SceneRoot(
    //         asset_server.load(
    //             GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")
    //         )
    //     )
    // );
}
