use std::{collections::HashMap, path::Path, rc::Rc};

use glium::{glutin::surface::WindowSurface, texture::RawImage2d, Display, Program, Texture2d};

use crate::{mesh::Mesh, ShaderDescriptor, Vertex};

#[derive(PartialEq, Eq, Hash)]
pub enum InternalMesh {
    Triangle,
    Square,
    Cube,
}

#[derive(PartialEq, Eq, Hash)]
pub enum InternalTexture {
    Crate,
}

pub struct ResourceManager<'a> {
    display: &'a Display<WindowSurface>,

    meshes: HashMap<&'static Path, Rc<Mesh>>,
    internal_meshes: HashMap<InternalMesh, Rc<Mesh>>,
    shaders: HashMap<ShaderDescriptor, Rc<Program>>,
    internal_shader: Rc<Program>,
    textures: HashMap<&'static Path, Rc<Texture2d>>,
    internal_textures: HashMap<InternalTexture, Rc<Texture2d>>,
}

impl<'a> ResourceManager<'a> {
    pub fn new(display: &'a Display<WindowSurface>) -> Self {
        let mut internal_meshes = HashMap::new();
        internal_meshes.insert(InternalMesh::Triangle, Rc::new(gen_triangle(display)));
        internal_meshes.insert(InternalMesh::Square, Rc::new(gen_square(display)));
        internal_meshes.insert(InternalMesh::Cube, Rc::new(gen_cube(display)));

        let mut internal_textures = HashMap::new();
        internal_textures.insert(InternalTexture::Crate, Rc::new(gen_crate(display)));

        Self {
            display,
            meshes: HashMap::new(),
            internal_meshes,
            shaders: HashMap::new(),
            internal_shader: Rc::new(gen_internal_program(display)),
            textures: HashMap::new(),
            internal_textures,
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

    pub fn load_texture(&mut self, name: &Path) -> Rc<Texture2d> {
        if !self.textures.contains_key(name) {
            todo!()
        }

        self.textures.get(name).unwrap().clone()
    }

    pub fn load_internal_mesh(&self, mesh: &InternalMesh) -> Rc<Mesh> {
        self.internal_meshes.get(mesh).unwrap().clone()
    }

    pub fn load_internal_shader(&self) -> Rc<Program> {
        self.internal_shader.clone()
    }

    pub fn load_internal_texture(&self, texture: &InternalTexture) -> Rc<Texture2d> {
        self.internal_textures.get(texture).unwrap().clone()
    }
}

fn gen_internal_program(display: &Display<WindowSurface>) -> Program {
    Program::from_source(
        display,
        include_str!("internal/shaders/vertex.glsl"),
        include_str!("internal/shaders/fragment.glsl"),
        None,
    )
    .unwrap()
}

fn gen_triangle(display: &Display<WindowSurface>) -> Mesh {
    let triangle_verts = vec![
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0),
        Vertex::new(0.0, 0.5, 0.0, 1.0, 1.0),
    ];

    Mesh {
        vertex_buffer: glium::VertexBuffer::new(display, &triangle_verts)
            .unwrap()
            .into(),
        indices: None,
    }
}

fn gen_square(display: &Display<WindowSurface>) -> Mesh {
    let square_verts = vec![
        Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0),
        Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0),
    ];

    let square_indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];

    Mesh {
        vertex_buffer: glium::VertexBuffer::new(display, &square_verts)
            .unwrap()
            .into(),
        indices: Some(
            glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &square_indices,
            )
            .unwrap()
            .into(),
        ),
    }
}

fn gen_cube(display: &Display<WindowSurface>) -> Mesh {
    let cube_verts = vec![
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 0.0),
        Vertex::new(0.5, -0.5, -0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 1.0),
        Vertex::new(-0.5, 0.5, 0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
    ];

    Mesh {
        vertex_buffer: glium::VertexBuffer::new(display, &cube_verts)
            .unwrap()
            .into(),
        indices: None,
    }
}

fn gen_crate(display: &Display<WindowSurface>) -> Texture2d {
    let image = image::load_from_memory(include_bytes!("internal/textures/container.jpg"))
        .unwrap()
        .to_rgba8();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    Texture2d::new(display, image).unwrap()
}
