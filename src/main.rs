//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use bevy_wasm::{ scene, models::{ MeshType, MeshParameters }, interaction::{ mouse, keyboard } };
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse::mouse_input_system, keyboard::keyboard_input_system))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // plane
    scene::plane::spwan_plane(&mut commands, &mut meshes, &mut materials);
    //swpan cube
    // scene::props::spwan_prop(&mut commands, &mut meshes, &mut materials, MeshParameters {
    //     dimensions: MeshType::Cube {
    //         width: 11.0,
    //         height: 1.0,
    //         depth: 1.0,
    //     },
    //     color: Color::srgb(0.8, 0.7, 0.6),
    //     position: Vec3::new(0.0, 0.5, 0.0),
    // });
    // light
    scene::light::spwan_light(&mut commands);
    // camera
    scene::camera::spawn_camera(&mut commands);
}
