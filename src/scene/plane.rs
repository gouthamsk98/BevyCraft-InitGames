use bevy::prelude::*;
use std::f32::consts::PI;
#[derive(Component)]
pub struct Ground;

pub fn spwan_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Ground,
    ));
}
pub fn add_grid(mut gizmos: Gizmos) {
    gizmos.grid(
        Quat::from_rotation_x(PI / 2.0),
        UVec2::splat(20),
        Vec2::new(1.0, 1.0),
        // Light gray
        LinearRgba::gray(0.05)
    );
}
