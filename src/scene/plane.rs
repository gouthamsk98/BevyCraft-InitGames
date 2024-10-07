use bevy::prelude::*;
use std::f32::consts::PI;
use crate::models::CursorType;
#[derive(Component)]
pub struct Ground;

pub fn spwan_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
            material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
            ..default()
        },
        Ground,
    ));
}
pub fn add_frid(mut gizmos: Gizmos) {
    gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(PI / 2.0),
        UVec2::splat(20),
        Vec2::new(1.0, 1.0),
        // Light gray
        LinearRgba::gray(0.05)
    );
}
pub fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(
        ground.translation(),
        InfinitePlane3d::new(ground.up())
    ) else {
        return;
    };
    let point = ray.get_point(distance);
    render_cursor(CursorType::Hash, 10.0, gizmos, point, ground);
}
fn render_cursor(
    cursor_type: CursorType,
    size: f32,
    mut gizmos: Gizmos,
    point: Vec3,
    ground: &GlobalTransform
) {
    // create gizmos to render the cursor wrt mesh type
    match cursor_type {
        CursorType::Sphere => {
            gizmos.sphere(point + ground.up() * 0.01, Quat::IDENTITY, size, Color::WHITE);
        }
        CursorType::Cuboid => { todo!() }
        CursorType::Circle => {
            // Draw a circle at the cursor position.
            gizmos.circle(point + ground.up() * 0.01, ground.up(), size, Color::WHITE);
        }
        CursorType::Square => {
            // Draw lines between the corners to form the square.
            // Calculate the four corners of the square.
            let offset = ground.up() * 0.01;
            let top_left = point + offset + Vec3::new(-size / 2.0, 0.0, -size / 2.0);
            let top_right = point + offset + Vec3::new(size / 2.0, 0.0, -size / 2.0);
            let bottom_left = point + offset + Vec3::new(-size / 2.0, 0.0, size / 2.0);
            let bottom_right = point + offset + Vec3::new(size / 2.0, 0.0, size / 2.0);

            gizmos.line(top_left, top_right, Color::WHITE);
            gizmos.line(top_right, bottom_right, Color::WHITE);
            gizmos.line(bottom_right, bottom_left, Color::WHITE);
            gizmos.line(bottom_left, top_left, Color::WHITE);
        }
        CursorType::Hash => {
            // Calculate the positions for the hash symbol.
            let offset = ground.up() * 0.01;

            // Horizontal lines
            let top_left_h1 = point + offset + Vec3::new(-size / 2.0, 0.0, -size / 4.0);
            let top_right_h1 = point + offset + Vec3::new(size / 2.0, 0.0, -size / 4.0);
            let top_left_h2 = point + offset + Vec3::new(-size / 2.0, 0.0, size / 4.0);
            let top_right_h2 = point + offset + Vec3::new(size / 2.0, 0.0, size / 4.0);

            // Vertical lines
            let bottom_left_v1 = point + offset + Vec3::new(-size / 4.0, 0.0, -size / 2.0);
            let top_left_v1 = point + offset + Vec3::new(-size / 4.0, 0.0, size / 2.0);
            let bottom_left_v2 = point + offset + Vec3::new(size / 4.0, 0.0, -size / 2.0);
            let top_left_v2 = point + offset + Vec3::new(size / 4.0, 0.0, size / 2.0);

            // Draw the horizontal lines.
            gizmos.line(top_left_h1, top_right_h1, Color::WHITE);
            gizmos.line(top_left_h2, top_right_h2, Color::WHITE);

            // Draw the vertical lines.
            gizmos.line(bottom_left_v1, top_left_v1, Color::WHITE);
            gizmos.line(bottom_left_v2, top_left_v2, Color::WHITE);
        }
        CursorType::Plane => { todo!() }
    }
}
