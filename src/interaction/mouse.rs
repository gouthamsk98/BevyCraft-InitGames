// mouse.rs
use bevy::prelude::*;
use crate::{ models::{ MeshParameters, MeshType, ToolType }, scene };
use crate::web::get_tool_type;

pub fn mouse_input_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>
) {
    let tool_type = get_tool_type().into();
    if mouse_button_input.just_pressed(MouseButton::Left) {
        match tool_type {
            ToolType::Default => {}
            ToolType::None => {}
            ToolType::Select => {}
            ToolType::Move => {}
            ToolType::Rotate => {}
            ToolType::Scale => {}
        }
    }
}
