use glium::implement_vertex;

pub mod mesh;
pub mod renderer;
pub mod mesh_manager;

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
}
implement_vertex!(Vertex, position);

impl Vertex {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { position: [x, y, z] }
	}
}

pub const TRIANGLE_ID: &str = "internal::triangle";
pub const SQUARE_ID: &str = "internal::square";
pub const CUBE_ID: &str = "internal::cube";
