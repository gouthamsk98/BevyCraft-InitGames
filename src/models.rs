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
