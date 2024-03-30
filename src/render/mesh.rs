use glam::Mat4;
use glium::{index::IndexBufferAny, vertex::VertexBufferAny};

pub struct Mesh {
	pub(crate) vertex_buffer: VertexBufferAny,
	pub(crate) indices: Option<IndexBufferAny>,
}