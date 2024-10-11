//! A simple 3D scene with light shining over a cube sitting on a plane.
use wasm_bindgen::prelude::*;
use bevy::prelude::*;
use bevycraft::{ scene, models::{ MeshType, MeshParameters }, interaction };
use bevy_mod_picking::DefaultPickingPlugins;
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
            interaction::camera_controller::CameraControllerPlugin,
        ))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (scene::plane::add_frid, scene::plane::handle_element_interaction))
        .add_systems(Update, (
            interaction::mouse::mouse_input_system,
            interaction::keyboard::keyboard_input_system,
        ))
        .run();
}

/// set up a simple 3D scene
use bevy::{ prelude::*, render::camera::ScalingMode };
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    scene::camera::spawn_camera(&mut commands);
    scene::plane::spwan_plane(&mut commands, &mut meshes, &mut materials);
    scene::light::spwan_light(&mut commands);
}
