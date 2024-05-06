use glam::{Mat4, Quat, Vec3A};

pub struct TransformComponent {
    pub translation: Vec3A,
    pub rotation: Quat,
    pub scale: Vec3A,
}

impl TransformComponent {
    const IDENTITY: Self = Self {
                translation: Vec3A::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3A::ONE,
            };

    pub fn new() -> Self {
        Self::IDENTITY
    }

    pub fn from_mat4(mat: Mat4) -> Self {
        let (scale, rotation, translation) = mat.to_scale_rotation_translation();
        
        Self {
            scale: scale.into(),
            rotation,
            translation: translation.into()
        }
    }

    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            translation: Vec3A::new(x, y, z),
            ..Self::IDENTITY
        }
    }

    pub fn get_mat_array(&self) -> [[f32; 4]; 4] {
        Mat4::from_scale_rotation_translation(self.scale.into(), self.rotation, self.translation.into()).to_cols_array_2d()
    }
}
