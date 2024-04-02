use std::{collections::BTreeMap, rc::Rc};

use glium::{glutin::surface::WindowSurface, Display, Program};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct ShaderDescriptor {
	vertex: &'static str,
	fragment: &'static str,
	geometry: Option<&'static str>,
}

pub const DEFAULT_SHADERS: ShaderDescriptor = ShaderDescriptor {
    vertex: "internal::vertex.glsl",
    fragment: "internal::fragment.glsl",
    geometry: None,
};

pub struct ShaderManager {
	program_map: BTreeMap<ShaderDescriptor, Rc<Program>>
}

impl ShaderManager {
	pub fn new(display: &Display<WindowSurface>) -> Self {
		let mut new = Self { program_map: BTreeMap::new() };

		new.program_map.insert(DEFAULT_SHADERS, Rc::new(Self::gen_default_program(display)));

		new
	}

	pub fn get_shader(&mut self, descriptor: &ShaderDescriptor) -> Rc<Program> {
		if !self.program_map.contains_key(descriptor) {
			todo!()
		}

		self.program_map.get(descriptor).unwrap().clone()
	}

	fn gen_default_program(display: &Display<WindowSurface>) -> Program {
		Program::from_source(display, include_str!("../shaders/internal/vertex.glsl"), include_str!("../shaders/internal/fragment.glsl"), None).unwrap()
	}
}