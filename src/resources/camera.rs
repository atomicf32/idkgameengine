use glam::{Mat4, Quat, Vec3};

use crate::components::transform::TransformComponent;

pub struct CameraResource {
    fov: f32,
    projection: Mat4,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl CameraResource {
    pub fn new(fov: f32, aspect_ratio: f32) -> Self {
        Self {
            fov,
            projection: Mat4::perspective_infinite_lh(fov, aspect_ratio, 0.1),
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn resize(&mut self, aspect_ratio: f32) {
        self.projection = Mat4::perspective_infinite_lh(self.fov, aspect_ratio, 0.1);
    }

    pub fn transform(&self, model: &TransformComponent) -> [[f32; 4]; 4] {
        (
            self.projection *
            Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation).inverse() *
            Mat4::from_scale_rotation_translation(model.scale, model.rotation, model.translation)
        ).to_cols_array_2d()
    }
}
