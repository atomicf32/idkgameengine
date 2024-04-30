use crate::{DrawData, DrawDescriptor};

pub struct DrawComponent {
    pub inner: Box<dyn DrawData>
}
