use std::{collections::HashMap, path::Path, rc::Rc};

use glium::{glutin::surface::WindowSurface, Display, Program};

use crate::{mesh::Mesh, renderer::Renderer, ShaderDescriptor};

#[derive(PartialEq, Eq, Hash)]
pub enum InternalMesh {
	Triangle,
	Square,
	Cube
}

pub struct ResourceManager<'a> {
	display: &'a Display<WindowSurface>,

	meshes: HashMap<&'static Path, Rc<Mesh>>,
	internal_meshes: HashMap<InternalMesh, Rc<Mesh>>,
	shaders: HashMap<ShaderDescriptor, Rc<Program>>,
	internal_shader: Rc<Program>,
}

impl<'a> ResourceManager<'a> {
	pub fn new(display: &'a Display<WindowSurface>, renderer: &Renderer) -> Self {
		let mut internal_meshes = HashMap::new();
		internal_meshes.insert(InternalMesh::Triangle, Rc::new(renderer.gen_triangle()));
		internal_meshes.insert(InternalMesh::Square, Rc::new(renderer.gen_square()));
		internal_meshes.insert(InternalMesh::Cube, Rc::new(renderer.gen_cube()));

		Self {
			display,
			meshes: HashMap::new(),
			internal_meshes,
			shaders: HashMap::new(),
			internal_shader: Rc::new(renderer.gen_internal_program()),
		}
	}

	pub fn load_mesh(&mut self, name: &Path) -> Rc<Mesh> {
		if !self.meshes.contains_key(name) {
			todo!()
		}

		self.meshes.get(name).unwrap().clone()
	}

	pub fn load_shader(&mut self, descriptor: &ShaderDescriptor) -> Rc<Program> {
		if !self.shaders.contains_key(descriptor) {
			todo!()
		}

		self.shaders.get(descriptor).unwrap().clone()
	}

	pub fn load_internal_mesh(&self, mesh: &InternalMesh) -> Rc<Mesh> {
		self.internal_meshes.get(mesh).unwrap().clone()
	}

	pub fn load_internal_shader(&self) -> Rc<Program> {
		self.internal_shader.clone()
	}
}