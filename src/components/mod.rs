use brood::Registry;

use self::{render::RenderComponent, transform::TransformComponent};

pub mod render;
pub mod transform;

pub(crate) type Registry = Registry!(RenderComponent, TransformComponent);