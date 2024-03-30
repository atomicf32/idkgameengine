mod mesh;
pub mod renderer;

pub enum MeshType {
	External(ExternalMesh),
	Internal(InternalMesh),
}

pub struct ExternalMesh {
	mesh_name: String,
	vertex_name: String,
	fragment_name: String,
	geometry_name: Option<String>,
}

pub enum InternalMesh {
	Triangle,
	Square,
	Cube,
}
