// camera.rs
use bevy::prelude::*;

pub fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
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
