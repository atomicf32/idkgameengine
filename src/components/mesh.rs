use crate::render::MeshType;

pub struct RenderComponent(pub MeshType);

impl RenderComponent {
	pub fn new(mesh_type: MeshType) -> Self {
		Self(mesh_type)
	}
}