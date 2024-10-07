use bevy::prelude::*;

pub fn spwan_light(commands: &mut Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}
