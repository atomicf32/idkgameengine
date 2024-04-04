use glam::Mat4;

pub struct CameraResource {
	fov: f32,
	projection: Mat4,
	view: Mat4
}

impl CameraResource {
	pub fn new(fov: f32, aspect_ratio: f32) -> Self {
		Self {
			fov,
			projection: Mat4::perspective_infinite_lh(fov, aspect_ratio, 0.1),
			view: Mat4::IDENTITY
		}
	}

	pub fn resize(&mut self, aspect_ratio: f32) {
		self.projection = Mat4::perspective_infinite_lh(self.fov, aspect_ratio, 0.1);
	}

	pub fn transform(&self, model: Mat4) -> [[f32; 4]; 4] {
		(self.projection * self.view.inverse() * model).to_cols_array_2d()
	}
}
