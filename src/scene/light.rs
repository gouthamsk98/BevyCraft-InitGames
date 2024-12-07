use bevy::prelude::*;

pub fn spwan_light(commands: &mut Commands) {
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}
