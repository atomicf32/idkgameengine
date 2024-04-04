use std::rc::Rc;

use brood::{query::filter, registry, result, system::System, Views};
use glium::{glutin::surface::WindowSurface, uniform, Display, DrawParameters, Program, Surface};

use crate::{components::{render::RenderComponent, transform::TransformComponent}, mesh::Mesh, resources::camera::CameraResource, shader_manager::{ShaderDescriptor, ShaderManager}};

use super::mesh_manager::MeshManager;

pub struct Renderer {
	// Mesh Manager
	mesh_manager: MeshManager,
	shader_manager: ShaderManager,
	// Display
	display: Display<WindowSurface>,
}

impl Renderer {
	pub fn new(display: Display<WindowSurface>) -> Self {
		Self {
			mesh_manager: MeshManager::new(&display),
			shader_manager: ShaderManager::new(&display),
			display,
		}
	}

	pub fn get_mesh(&mut self, name: &str) -> Rc<Mesh> {
		self.mesh_manager.get_mesh(name)
	}

	pub fn get_shader(&mut self, descriptor: &ShaderDescriptor) -> Rc<Program> {
		self.shader_manager.get_shader(descriptor)
	}
}

impl System for Renderer {
	type Filter = filter::None;
	type Views<'a> = Views!(&'a TransformComponent, &'a RenderComponent);
	type ResourceViews<'a> = Views!(&'a CameraResource);
	type EntryViews<'a> = Views!();

	fn run<'a, R, S, I, E>(
		&mut self,
		query_result: brood::query::Result<'a, R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>,
	) where
		R: registry::Registry,
		I: Iterator<Item = Self::Views<'a>> 
	{
		let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

		// TODO: Fix the depth buffer
		let draw_parameters = DrawParameters {
			depth: glium::Depth { test: glium::draw_parameters::DepthTest::IfLess, write: true, ..Default::default() },
			..Default::default()
		};

		let result!(camera) = query_result.resources;

		for result!(transform, render) in query_result.iter {
			match &render.mesh.indices {
				Some(i) => {
					target.draw(
						&render.mesh.vertex_buffer,
						i,
						&render.shader.as_ref(),
						&uniform! {
							matrix: camera.transform(transform.inner())
						},
						&draw_parameters
					).unwrap();
				},
				None => {
					target.draw(
						&render.mesh.vertex_buffer,
						glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
						&render.shader.as_ref(),
						&uniform! {
							matrix: camera.transform(transform.inner())
						},
						&draw_parameters
					).unwrap();
				},
			}
		}

        target.finish().unwrap();
	}
}