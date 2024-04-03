use std::{collections::HashMap, rc::Rc};

use glium::{glutin::surface::WindowSurface, Display};

use super::{mesh::Mesh, Vertex};

pub const TRIANGLE_ID: &str = "internal::triangle";
pub const SQUARE_ID: &str = "internal::square";
pub const CUBE_ID: &str = "internal::cube";

pub struct MeshManager {
	mesh_map: HashMap<&'static str, Rc<Mesh>>,
}

impl MeshManager {
	pub fn new(display: &Display<WindowSurface>) -> Self {
		let mut new = Self {
			mesh_map: HashMap::new()
		};

		new.mesh_map.insert(TRIANGLE_ID, Rc::new(Self::gen_triangle(display)));
		new.mesh_map.insert(SQUARE_ID, Rc::new(Self::gen_square(display)));
		new.mesh_map.insert(CUBE_ID, Rc::new(Self::gen_cube(display)));

		new
	}

	pub fn get_mesh(&mut self, name: &str) -> Rc<Mesh> {
		if !self.mesh_map.contains_key(name) {
			// TODO: make it load files
			todo!()
		}

		self.mesh_map.get(name).unwrap().clone()
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
}

