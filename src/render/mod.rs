use std::path::Path;

use glium::implement_vertex;

pub mod mesh;
pub mod renderer;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Self {
            position: [x, y, z],
            tex_coords: [u, v],
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ShaderDescriptor {
    vertex: &'static Path,
    fragment: &'static Path,
    geometry: Option<&'static Path>,
}
