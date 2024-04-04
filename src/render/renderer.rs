use brood::{query::filter, registry, result, system::System, Views};
use glium::{glutin::surface::WindowSurface, uniform, Display, DrawParameters, Program, Surface};

use crate::{components::{draw::DrawComponent, transform::TransformComponent}, mesh::Mesh, resources::camera::CameraResource, Vertex};

pub struct Renderer<'a> {
	// Display
	display: &'a Display<WindowSurface>,
}

impl<'a> Renderer<'a> {
	pub fn new(display: &'a Display<WindowSurface>) -> Self {
		Self {
			display,
		}
	}

	pub fn gen_internal_program(&self) -> Program {
		Program::from_source(self.display, include_str!("../shaders/internal/vertex.glsl"), include_str!("../shaders/internal/fragment.glsl"), None).unwrap()
	}

	pub fn gen_triangle(&self) -> Mesh {
		let triangle_verts = vec![
			Vertex::new(-0.5, -0.5, 0.5),
			Vertex::new( 0.5, -0.5, 0.5),
			Vertex::new( 0.0,  0.5, 0.5),
		];

		Mesh {
			vertex_buffer: glium::VertexBuffer::new(self.display, &triangle_verts).unwrap().into(),
    		indices: None,
		}
	}

	pub fn gen_square(&self) -> Mesh {
		let square_verts = vec![
			Vertex::new( 0.5,  0.5, 0.0),
			Vertex::new( 0.5, -0.5, 0.0),
			Vertex::new(-0.5, -0.5, 0.0),
			Vertex::new(-0.5,  0.5, 0.0),
		];

		let square_indices: Vec<u32> = vec![
			0, 1, 3,
        	1, 2, 3
		];

		Mesh {
			vertex_buffer: glium::VertexBuffer::new(self.display, &square_verts).unwrap().into(),
			indices: Some(glium::IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList, &square_indices).unwrap().into()),
		}
	}

	pub fn gen_cube(&self) -> Mesh {
		let cube_verts = vec![
			Vertex::new(-0.5, -0.5, -0.5),
        	Vertex::new( 0.5, -0.5, -0.5),
        	Vertex::new( 0.5,  0.5, -0.5),
        	Vertex::new( 0.5,  0.5, -0.5),
        	Vertex::new(-0.5,  0.5, -0.5),
        	Vertex::new(-0.5, -0.5, -0.5),
        	Vertex::new(-0.5, -0.5,  0.5),
        	Vertex::new( 0.5, -0.5,  0.5),
        	Vertex::new( 0.5,  0.5,  0.5),
        	Vertex::new( 0.5,  0.5,  0.5),
        	Vertex::new(-0.5,  0.5,  0.5),
        	Vertex::new(-0.5, -0.5,  0.5),
        	Vertex::new(-0.5,  0.5,  0.5),
        	Vertex::new(-0.5,  0.5, -0.5),
        	Vertex::new(-0.5, -0.5, -0.5),
        	Vertex::new(-0.5, -0.5, -0.5),
        	Vertex::new(-0.5, -0.5,  0.5),
        	Vertex::new(-0.5,  0.5,  0.5),
        	Vertex::new( 0.5,  0.5,  0.5),
        	Vertex::new( 0.5,  0.5, -0.5),
        	Vertex::new( 0.5, -0.5, -0.5),
        	Vertex::new( 0.5, -0.5, -0.5),
        	Vertex::new( 0.5, -0.5,  0.5),
        	Vertex::new( 0.5,  0.5,  0.5),
        	Vertex::new(-0.5, -0.5, -0.5),
        	Vertex::new( 0.5, -0.5, -0.5),
        	Vertex::new( 0.5, -0.5,  0.5),
        	Vertex::new( 0.5, -0.5,  0.5),
        	Vertex::new(-0.5, -0.5,  0.5),
        	Vertex::new(-0.5, -0.5, -0.5),
        	Vertex::new(-0.5,  0.5, -0.5),
        	Vertex::new( 0.5,  0.5, -0.5),
        	Vertex::new( 0.5,  0.5,  0.5),
        	Vertex::new( 0.5,  0.5,  0.5),
        	Vertex::new(-0.5,  0.5,  0.5),
        	Vertex::new(-0.5,  0.5, -0.5)
		];

		Mesh {
			vertex_buffer: glium::VertexBuffer::new(self.display, &cube_verts).unwrap().into(),
    		indices: None,
		}
	}
}

impl System for Renderer<'_> {
	type Filter = filter::None;
	type Views<'a> = Views!(&'a TransformComponent, &'a DrawComponent);
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
							matrix: camera.transform(*transform.inner())
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
							matrix: camera.transform(*transform.inner())
						},
						&draw_parameters
					).unwrap();
				},
			}
		}

        target.finish().unwrap();
	}
}