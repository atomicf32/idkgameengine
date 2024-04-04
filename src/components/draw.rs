use std::{path::Path, rc::Rc};

use glium::Program;

use crate::{render::mesh::Mesh, resource_manager::{InternalMesh, ResourceManager}, ShaderDescriptor};

pub enum MeshType {
	Internal(InternalMesh),
	External(&'static Path),
}

pub enum ShaderType {
	Internal,
	External(ShaderDescriptor),
}

pub struct DrawDescriptor {
	mesh: MeshType,
	shader: ShaderType,
}

impl Default for DrawDescriptor {
	fn default() -> Self {
		Self { mesh: MeshType::Internal(InternalMesh::Cube), shader: ShaderType::Internal }
	}
}

pub struct DrawComponent {
	pub mesh: Rc<Mesh>,
	pub shader: Rc<Program>,
}

impl DrawComponent {
	pub fn new(mesh: Rc<Mesh>, shader: Rc<Program>) -> Self {
		Self {
			mesh,
			shader
		}
	}

	pub fn load(manager: &mut ResourceManager, descriptor: &DrawDescriptor) -> Self {
		Self {
			mesh: match descriptor.mesh {
				MeshType::Internal(ref i) => manager.load_internal_mesh(i),
				MeshType::External(i) => manager.load_mesh(i),
			},
			shader: match descriptor.shader {
				ShaderType::Internal => manager.load_internal_shader(),
				ShaderType::External(ref i) => manager.load_shader(i),
			},
		}
	}
}
