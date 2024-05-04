use glam::{Mat4, Quat, Vec3A};

use crate::components::transform::TransformComponent;

pub struct CameraResource {
    fov: f32,
    projection: Mat4,
    pub translation: Vec3A,
    pub rotation: Quat,
    pub scale: Vec3A,
}

impl CameraResource {
    pub fn new(fov: f32, aspect_ratio: f32) -> Self {
        Self {
            fov,
            projection: Mat4::perspective_infinite_lh(fov, aspect_ratio, 0.1),
            translation: Vec3A::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3A::ONE,
        }
    }

    pub fn resize(&mut self, aspect_ratio: f32) {
        self.projection = Mat4::perspective_infinite_lh(self.fov, aspect_ratio, 0.1);
    }

    pub fn transform_model(&self, model: &TransformComponent) -> [[f32; 4]; 4] {
        (
            self.projection *
            Mat4::from_scale_rotation_translation(self.scale.into(), self.rotation, self.translation.into()).inverse() *
            Mat4::from_scale_rotation_translation(model.scale.into(), model.rotation, model.translation.into())
        ).to_cols_array_2d()
    }
}
