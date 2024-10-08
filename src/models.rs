use bevy::prelude::*;

pub enum MeshType {
    Cube {
        width: f32,
        height: f32,
        depth: f32,
    },
    Sphere {
        radius: f32,
    },
    Cylinder {
        radius: f32,
        height: f32,
    },
    Capsule3D {
        radius: f32,
        height: f32,
    },
    Plane3D {
        width: f32,
        height: f32,
    },
}
pub struct MeshParameters {
    // pub mesh_type: MeshType,
    pub dimensions: MeshType,
    pub color: Color,
    pub position: Vec3,
}
pub enum CursorType {
    Sphere,
    Cuboid,
    Circle,
    Square,
    Plane,
    Custom,
}
impl From<String> for CursorType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Sphere" => CursorType::Sphere,
            "Cuboid" => CursorType::Cuboid,
            "Circle" => CursorType::Circle,
            "Square" => CursorType::Square,
            "Plane" => CursorType::Plane,
            "Custom" => CursorType::Custom,
            _ => CursorType::Custom, // Default to Sphere if unknown
        }
    }
}
#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_cursor_grab: MouseButton,
    pub keyboard_key_toggle_cursor_grab: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub scroll_factor: f32,
    pub scroll: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}
impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 1.0,
            key_forward: KeyCode::KeyW,
            key_back: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyE,
            key_down: KeyCode::KeyQ,
            key_run: KeyCode::ShiftLeft,
            mouse_key_cursor_grab: MouseButton::Left,
            keyboard_key_toggle_cursor_grab: KeyCode::KeyM,
            walk_speed: 5.0,
            run_speed: 15.0,
            scroll_factor: 0.1,
            scroll: 1.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}
