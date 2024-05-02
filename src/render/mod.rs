use std::{any::Any, borrow::Cow, path::Path};

use brood::World;

use crate::{
    components::{draw::DrawComponent, Registry},
    resources::Resources,
};

pub mod ogl_renderer;

pub trait Renderer {
    fn render(&mut self, world: &mut World<Registry, Resources>);
    fn load(&mut self, descriptor: &DrawDescriptor) -> DrawComponent;
}

pub trait DrawData {
    fn as_any(&self) -> &dyn Any;
}

pub struct DrawDescriptor {
    pub mesh: Mesh,
    pub texture: Cow<'static, Path>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Mesh {
    Triangle,
    Square,
    Cube,
    Gltf(Cow<'static, Path>),
}
