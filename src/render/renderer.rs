use brood::{query::filter, registry, result, system::System, Views};
use glam::Mat4;
use glium::{glutin::surface::WindowSurface, implement_vertex, uniform, Display, Program, Surface};

use crate::components::{mesh::RenderComponent, transform::TransformComponent};

use super::mesh::Mesh;
use crate::render::MeshType::{ Internal, External };

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

pub struct Renderer {
	display: Display<WindowSurface>,
	triangle: Mesh,
	square: Mesh,
	cube: Mesh,
	default_program: Program,
	proj_mat: Mat4,
	view_mat: Mat4,
}

impl Renderer {
	pub fn new(display: Display<WindowSurface>) -> Self {
		let triangle = Self::gen_triangle(&display);
		let square = Self::gen_square(&display);
		let cube = Self::gen_cube(&display);
		let default_program = Self::gen_default_program(&display);
		let aspect_ratio = display.get_framebuffer_dimensions().0 as f32 / display.get_framebuffer_dimensions().1 as f32;
		let proj_mat = Mat4::perspective_lh(45_f32.to_radians(), aspect_ratio, 0.0, 100.0);
		let view_mat = Mat4::IDENTITY;

		Self {
			display,
			triangle,
			square,
			cube,
			default_program,
			proj_mat,
			view_mat,
		}
	}

	fn gen_triangle(display: &Display<WindowSurface>) -> Mesh {
		let triangle_verts = vec![
			Vertex::new(-0.5, -0.5, 0.5),
			Vertex::new( 0.5, -0.5, 0.5),
			Vertex::new( 0.0,  0.5, 0.5),
		];

		Mesh {
			vertex_buffer: glium::VertexBuffer::new(display, &triangle_verts).unwrap().into(),
    		indices: None,
		}
	}

	fn gen_square(display: &Display<WindowSurface>) -> Mesh {
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
			vertex_buffer: glium::VertexBuffer::new(display, &square_verts).unwrap().into(),
			indices: Some(glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &square_indices).unwrap().into()),
		}
	}

	fn gen_cube(display: &Display<WindowSurface>) -> Mesh {
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
			vertex_buffer: glium::VertexBuffer::new(display, &cube_verts).unwrap().into(),
    		indices: None,
		}
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
		let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

		for result!(transform, render) in query_result.iter {
			match render.0 {
    			Internal(ref i) => {
					let mesh = match i {
						super::InternalMesh::Triangle => &self.triangle,
						super::InternalMesh::Square => &self.square,
						super::InternalMesh::Cube => &self.cube,
					};

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
									&Default::default()
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
									&Default::default()
								).unwrap();
							},
						}
				}
				External(ref i) => {

				}
			}
		}

        target.finish().unwrap();
	}
}