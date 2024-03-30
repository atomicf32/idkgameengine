use glam::Mat4;

pub struct TransformComponent(pub Mat4);

impl TransformComponent {
	pub fn new() -> Self {
		Self(Mat4::IDENTITY)
	}

	pub fn from_mat4(mat: Mat4) -> Self {
		Self(mat)
	}
}