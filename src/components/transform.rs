use glam::{Mat4, Quat, Vec3};

pub struct TransformComponent {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl TransformComponent {
    const IDENTITY: Self = Self {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            };

    pub fn new() -> Self {
        Self::IDENTITY
    }

    pub fn from_mat4(mat: Mat4) -> Self {
        let (scale, rotation, translation) = mat.to_scale_rotation_translation();
        
        Self {
            scale,
            rotation,
            translation
        }
    }

    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            translation: Vec3::new(x, y, z),
            ..Self::IDENTITY
        }
    }
}
