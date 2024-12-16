use bevy::prelude::*;
use serde::Serialize;

//
#[derive(Serialize)]
pub struct MeshPrams {
    pub mesh_type: String,
    pub color: Color,
    pub position: Vec3,
}
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
impl From<String> for MeshType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Cube" =>
                MeshType::Cube {
                    width: 1.0,
                    height: 1.0,
                    depth: 1.0,
                },
            "Sphere" =>
                MeshType::Sphere {
                    radius: 0.5,
                },
            "Cylinder" =>
                MeshType::Cylinder {
                    radius: 0.5,
                    height: 1.0,
                },
            "Capsule3D" =>
                MeshType::Capsule3D {
                    radius: 0.5,
                    height: 1.0,
                },
            "Plane3D" =>
                MeshType::Plane3D {
                    width: 1.0,
                    height: 1.0,
                },
            _ =>
                MeshType::Cube {
                    width: 1.0,
                    height: 1.0,
                    depth: 1.0,
                },
        }
    }
}
pub struct MeshParameters {
    // pub mesh_type: MeshType,
    pub dimensions: MeshType,
    pub color: Color,
    pub position: Vec3,
}
#[derive(PartialEq)]
pub enum CursorType {
    Default,
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
            "Default" => CursorType::Default,
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
pub struct TouchCursor {
    pub cursor_type: CursorType,
    pub color: Color,
    pub position: Vec3,
    pub scale: Vec3,
}
impl Default for TouchCursor {
    fn default() -> Self {
        Self {
            cursor_type: CursorType::Default,
            color: Color::WHITE,
            position: Vec3::ZERO,
            scale: Vec3::ONE,
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
    pub mouse_left_key_cursor_grab: MouseButton,
    pub mouse_right_key_cursor_grab: MouseButton,
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
            mouse_left_key_cursor_grab: MouseButton::Left,
            mouse_right_key_cursor_grab: MouseButton::Right,
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
#[derive(PartialEq)]
pub enum ToolType {
    Select,
    Move,
    Rotate,
    Scale,
    Default,
    None,
}
impl Default for ToolType {
    fn default() -> Self {
        ToolType::Default
    }
}
impl From<String> for ToolType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Select" => ToolType::Select,
            "Move" => ToolType::Move,
            "Rotate" => ToolType::Rotate,
            "Scale" => ToolType::Scale,
            "Default" => ToolType::Default,
            _ => ToolType::None,
        }
    }
}

#[derive(Component)]
pub struct MeshId(pub u32);

impl Default for MeshId {
    fn default() -> Self {
        Self(0)
    }
}
#[derive(Resource)]
pub struct CurrentMeshEntity(pub Option<Entity>);
impl Default for CurrentMeshEntity {
    fn default() -> Self {
        Self(None)
    }
}
