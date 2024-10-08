//! A simple 3D scene with light shining over a cube sitting on a plane.
use wasm_bindgen::prelude::*;
use bevy::prelude::*;
use bevycraft::{ scene, models::{ MeshType, MeshParameters }, interaction };
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, interaction::camera_controller::CameraControllerPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (scene::plane::add_frid, scene::plane::draw_cursor))

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
