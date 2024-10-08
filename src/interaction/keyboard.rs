use bevy::prelude::*;
use crate::scene;
//chnaget the camera from 2d to 3d view with key press
pub fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera: Query<Entity, With<Camera>>,
    mut commands: Commands
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Space key pressed");
        todo!();
    }
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        println!("C key pressed");
        todo!()
    }
}
