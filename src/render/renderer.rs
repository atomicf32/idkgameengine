use std::rc::Rc;

use brood::{query::filter, registry, result, system::System, Views};
use glam::Mat4;
use glium::{glutin::surface::WindowSurface, uniform, Display, DrawParameters, Program, Surface};

use crate::{components::{render::RenderComponent, transform::TransformComponent}, mesh::Mesh};

use super::mesh_manager::MeshManager;

pub struct Renderer {
	display: Display<WindowSurface>,
	// Default meshes
	default_program: Program,
	// Matrices
	proj_mat: Mat4,
	view_mat: Mat4,
	// Mesh Manager
	mesh_manager: MeshManager,
}

impl Renderer {
	pub fn new(display: Display<WindowSurface>) -> Self {
		let default_program = Self::gen_default_program(&display);
		let aspect_ratio = display.get_framebuffer_dimensions().0 as f32 / display.get_framebuffer_dimensions().1 as f32;
		let proj_mat = Mat4::perspective_lh(45_f32.to_radians(), aspect_ratio, 0.0, 100.0);
		let view_mat = Mat4::IDENTITY;
		let mesh_manager = MeshManager::new(&display);

		Self {
			display,
			default_program,
			proj_mat,
			view_mat,
			mesh_manager,
		}
	}

	pub fn get_mesh(&mut self, name: &str) -> Rc<Mesh> {
		self.mesh_manager.get_mesh(name)
	}

	fn gen_default_program(display: &Display<WindowSurface>) -> Program {
		Program::from_source(display, include_str!("../shaders/vertex.glsl"), include_str!("../shaders/fragment.glsl"), None).unwrap()
	}
}

impl System for Renderer {
	type Filter = filter::None;
	type Views<'a> = Views!(&'a TransformComponent, &'a RenderComponent);
	type ResourceViews<'a> = Views!();
	type EntryViews<'a> = Views!();

	fn run<'a, R, S, I, E>(
		&mut self,
		query_result: brood::query::Result<'a, R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>,
	) where
		R: registry::Registry,
		I: Iterator<Item = Self::Views<'a>> 
	{
		// TODO: Fix the depth buffer
		let draw_parameters = DrawParameters {
			depth: glium::Depth { test: glium::draw_parameters::DepthTest::IfMoreOrEqual, write: true, ..Default::default() },
			..Default::default()
		};

		let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

		for result!(transform, render) in query_result.iter {
			let mesh = &render.0;

			match &mesh.indices {
				Some(i) => {
					target.draw(
						&mesh.vertex_buffer,
						i,
						&self.default_program,
						&uniform! {
							proj: self.proj_mat.to_cols_array_2d(),
							view: self.view_mat.to_cols_array_2d(),
							model: transform.0.to_cols_array_2d()
						},
						&draw_parameters
					).unwrap();
				},
				None => {
					target.draw(
						&mesh.vertex_buffer,
						glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
						&self.default_program,
						&uniform! {
							proj: self.proj_mat.to_cols_array_2d(),
							view: self.view_mat.to_cols_array_2d(),
							model: transform.0.to_cols_array_2d(),
						},
						&draw_parameters
					).unwrap();
				},
			}
		}

        target.finish().unwrap();
	}
}