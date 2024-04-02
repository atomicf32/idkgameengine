use glam::Mat4;

pub struct CameraResource {
	fov: f32,
	pub projection: Mat4,
	pub view: Mat4
}

impl CameraResource {
	pub fn new(fov: f32, aspect_ratio: f32) -> Self {
		Self {
			fov,
			projection: Mat4::perspective_infinite_lh(fov, aspect_ratio, 0.0),
			view: Mat4::IDENTITY
		}
	}

	pub fn resize(&mut self, aspect_ratio: f32) {
		self.projection = Mat4::perspective_infinite_lh(self.fov, aspect_ratio, 0.0);
	}
}
