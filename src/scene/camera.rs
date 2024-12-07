// camera.rs

use bevy::{ prelude::*, render::camera::ScalingMode };
use crate::models::{ CameraController, ToolType };
pub fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));
}
