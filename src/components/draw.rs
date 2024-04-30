use crate::DrawData;

pub struct DrawComponent {
    pub inner: Box<dyn DrawData>
}
