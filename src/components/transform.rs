use glam::{Mat4, Vec3};

pub struct TransformComponent(pub Mat4);

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
}