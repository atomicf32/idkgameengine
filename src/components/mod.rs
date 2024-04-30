use brood::Registry;

use self::{draw::DrawComponent, light::LightComponent, transform::TransformComponent};

pub mod draw;
pub mod light;
pub mod transform;

pub type Registry = Registry!(DrawComponent, LightComponent, TransformComponent);
