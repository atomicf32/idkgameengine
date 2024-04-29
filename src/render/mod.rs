use std::any::Any;

use brood::World;
use winit::window::Window;

use crate::{components::{draw::DrawComponent, Registry}, resources::Resources};

pub mod ogl_renderer;
pub trait Renderer {
    fn get_window(&self) -> &Window;
    fn render(&mut self, world: &mut World<Registry, Resources>);
    fn load(&mut self, mesh_name: &str, texture_name: &str) -> DrawComponent;
}

pub trait DrawData {
    fn as_any(&self) -> &dyn Any;
}