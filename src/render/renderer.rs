use brood::{query::filter, registry, result, system::System, Views};
use glium::{glutin::surface::WindowSurface, uniform, Display, DrawParameters, Surface};

use crate::{
    components::{draw::DrawComponent, transform::TransformComponent},
    resources::camera::CameraResource,
};

pub struct Renderer<'a> {
    // Display
    display: &'a Display<WindowSurface>,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a Display<WindowSurface>) -> Self {
        Self { display }
    }
}

impl System for Renderer<'_> {
    type Filter = filter::None;
    type Views<'a> = Views!(&'a TransformComponent, &'a DrawComponent);
    type ResourceViews<'a> = Views!(&'a CameraResource);
    type EntryViews<'a> = Views!();

    fn run<'a, R, S, I, E>(
        &mut self,
        query_result: brood::query::Result<
            'a,
            R,
            S,
            I,
            Self::ResourceViews<'a>,
            Self::EntryViews<'a>,
            E,
        >,
    ) where
        R: registry::Registry,
        I: Iterator<Item = Self::Views<'a>>,
    {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let draw_parameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let result!(camera) = query_result.resources;

        for result!(transform, render) in query_result.iter {
            let (mesh, shader, texture) = render.get_all();

            match &mesh.indices {
                Some(i) => {
                    target
                        .draw(
                            &mesh.vertex_buffer,
                            i,
                            &shader.as_ref(),
                            &uniform! {
                                matrix: camera.transform(*transform.inner()),
                                tex: texture.as_ref(),
                            },
                            &draw_parameters,
                        )
                        .unwrap();
                }
                None => {
                    target
                        .draw(
                            &mesh.vertex_buffer,
                            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                            &shader.as_ref(),
                            &uniform! {
                                matrix: camera.transform(*transform.inner()),
                                tex: texture.as_ref(),
                            },
                            &draw_parameters,
                        )
                        .unwrap();
                }
            }
        }

        target.finish().unwrap();
    }
}
