use brood::Registry;

use self::{render::RenderComponent, transform::TransformComponent};

pub mod render;
pub mod transform;

pub type Registry = Registry!(RenderComponent, TransformComponent);