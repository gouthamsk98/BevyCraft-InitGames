use bevy::prelude::*;
use std::f32::consts::PI;
use crate::models::CursorType;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    pub fn get_cursor_type() -> String;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_cursor_type() -> String {
    "Sphere".to_string()
}

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
    gizmos: Gizmos
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
    let cursor_type_str = get_cursor_type();
    let cursor_type: CursorType = cursor_type_str.into();
    render_cursor(cursor_type, 1.0, gizmos, point, ground);
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
            gizmos.sphere(point + ground.up() * 0.01, Quat::IDENTITY, size / 2.0, Color::WHITE);
        }
        CursorType::Cuboid => {
            let transform = Transform::from_translation(point + ground.up() * 0.01).with_scale(
                Vec3::splat(size)
            );
            gizmos.cuboid(transform, Color::WHITE);
        }
        CursorType::Circle => {
            // Draw a circle at the cursor position.
            gizmos.circle(point + ground.up() * 0.01, ground.up(), size / 2.0, Color::WHITE);
        }
        CursorType::Square => {
            gizmos.rect(
                point + ground.up() * 0.01,
                Quat::from_rotation_x(PI / 2.0),
                Vec2::splat(size),
                Color::WHITE
            );
        }
        CursorType::Plane => {
            gizmos.grid(
                point + ground.up() * 0.01,
                Quat::IDENTITY,
                UVec2::splat(10),
                Vec2::splat(size / 10.0),
                Color::WHITE
            );
        }
        CursorType::Custom => {
            gizmos.grid_3d(
                point + ground.up() * 0.01,
                Quat::IDENTITY,
                UVec3::splat(10),
                Vec3::splat(size / 10.0),
                Color::WHITE
            );
        }
    }
}
