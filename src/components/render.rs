use std::rc::Rc;

use glium::Program;

use crate::{render::mesh::Mesh, renderer::Renderer, shader_manager::ShaderDescriptor};

pub struct RenderComponent {
	pub mesh: Rc<Mesh>,
	pub shader: Rc<Program>,
}

impl RenderComponent {
	pub fn new(mesh: Rc<Mesh>, shader: Rc<Program>) -> Self {
		Self {
			mesh,
			shader
		}
	}

	pub fn from_renderer(renderer: &mut Renderer, mesh_name: &str, shader_descriptor: &ShaderDescriptor) -> Self {
		Self {
			mesh: renderer.get_mesh(mesh_name),
			shader: renderer.get_shader(shader_descriptor)
		}
	}
}
