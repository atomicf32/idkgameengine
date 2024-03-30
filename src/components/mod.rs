use brood::Registry;

use self::{mesh::RenderComponent, transform::TransformComponent};

pub mod mesh;
pub mod transform;

pub(crate) type Registry = Registry!(RenderComponent, TransformComponent);