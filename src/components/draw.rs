use std::{path::Path, rc::Rc};

use glium::{Program, Texture2d};

use crate::{
    render::mesh::Mesh,
    resource_manager::{InternalMesh, InternalTexture, ResourceManager},
    ShaderDescriptor,
};

pub enum MeshType {
    Internal(InternalMesh),
    External(&'static Path),
}

pub enum ShaderType {
    Internal,
    External(ShaderDescriptor),
}

pub enum TextureType {
    Internal(InternalTexture),
    External(&'static Path),
}

pub struct DrawDescriptor {
    mesh: MeshType,
    shader: ShaderType,
    texture: TextureType,
}

impl Default for DrawDescriptor {
    fn default() -> Self {
        Self {
            mesh: MeshType::Internal(InternalMesh::Cube),
            shader: ShaderType::Internal,
            texture: TextureType::Internal(InternalTexture::Crate),
        }
    }
}

pub struct DrawComponent {
    mesh: Rc<Mesh>,
    shader: Rc<Program>,
    texture: Rc<Texture2d>,
}

impl DrawComponent {
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
            texture: match descriptor.texture {
                TextureType::Internal(ref i) => manager.load_internal_texture(i),
                TextureType::External(i) => manager.load_texture(i),
            },
        }
    }

    pub fn get_mesh(&self) -> Rc<Mesh> {
        self.mesh.clone()
    }

    pub fn get_shader(&self) -> Rc<Program> {
        self.shader.clone()
    }

    pub fn get_texture(&self) -> Rc<Texture2d> {
        self.texture.clone()
    }

    pub fn get_all(&self) -> (Rc<Mesh>, Rc<Program>, Rc<Texture2d>) {
        (self.mesh.clone(), self.shader.clone(), self.texture.clone())
    }
}
