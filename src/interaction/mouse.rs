// mouse.rs
use bevy::prelude::*;
use crate::{ scene, models::{ MeshParameters, MeshType } };

pub fn mouse_input_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>
) {
    //confdition to check if the left mouse button is pressed on a mesh

    // if mouse_button_input.just_pressed(MouseButton::Left) {
    //     //change the mesh material color on mouse click with mesh in place
    //     let params = MeshParameters {
    //         dimensions: MeshType::Cube {
    //             width: 1.0,
    //             height: 1.0,
    //             depth: 1.0,
    //         },
    //         color: Color::srgb(0.8, 0.7, 0.6),
    //         position: Vec3::new(0.0, 0.5, 0.0),
    //     };
    //     scene::props::spwan_prop(&mut commands, &mut meshes, &mut materials, params);

    // }
}
