// camera.rs

use bevy::{ prelude::*, render::camera::ScalingMode };
use crate::models::CameraController;
pub fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: (OrthographicProjection {
                // 6 world units per window height.
                scaling_mode: ScalingMode::FixedVertical(5.0),
                ..default()
            }).into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));
}
