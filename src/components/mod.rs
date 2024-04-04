use brood::Registry;

use self::{draw::DrawComponent, transform::TransformComponent};

pub mod draw;
pub mod transform;

pub type Registry = Registry!(DrawComponent, TransformComponent);