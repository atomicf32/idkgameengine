use glam::{Mat4, Vec3};

pub struct TransformComponent(Mat4);

impl TransformComponent {
	pub fn new() -> Self {
		Self(Mat4::IDENTITY)
	}

	pub fn from_mat4(mat: Mat4) -> Self {
		Self(mat)
	}

	pub fn from_position(x: f32, y: f32, z: f32) -> Self {
		Self(Mat4::from_translation(Vec3::new(x, y, z)))
	}

	pub fn inner(&self) -> Mat4 {
		self.0
	}

	pub fn transform_global(&mut self, mat: Mat4) {
		self.0 = mat * self.0;
	}

	pub fn transform_local(&mut self, mat: Mat4) {
		self.0 *= mat;
	}
}