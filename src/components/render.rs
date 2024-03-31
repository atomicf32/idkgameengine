use std::rc::Rc;

use crate::render::mesh::Mesh;

pub struct RenderComponent(pub Rc<Mesh>);
