// camera.rs

use bevy::{ prelude::*, render::camera::ScalingMode };

pub fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        projection: (OrthographicProjection {
            // 6 world units per window height.
            scaling_mode: ScalingMode::FixedVertical(5.0),
            ..default()
        }).into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

//function to change the camera 3d to 2d view
pub fn change_camera_2d(commands: &mut Commands, camera: &Query<Entity, With<Camera>>) {
    for entity in camera.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn(Camera2dBundle::default());
}
pub fn change_camera_3d(commands: &mut Commands, camera: &Query<Entity, With<Camera>>) {
    for entity in camera.iter() {
        commands.entity(entity).despawn();
    }
    spawn_camera(commands);
}
