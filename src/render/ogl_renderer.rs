use std::{collections::HashMap, fs, num::NonZeroU32, rc::{Rc, Weak}};

use brood::{query::filter, registry, result, system::System, Views};
use glium::{glutin::surface::WindowSurface, implement_vertex, texture::RawImage2d, uniform, Display, DrawParameters, Program, Surface, Texture2d};
use glutin::{config::ConfigTemplateBuilder, context::NotCurrentGlContext, display::{GetGlDisplay, GlDisplay}};
use raw_window_handle::HasRawWindowHandle;
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}};
use glium::{index::IndexBufferAny, vertex::VertexBufferAny};
use crate::{components::{draw::DrawComponent, transform::TransformComponent}, resources::camera::CameraResource, DrawData, Renderer};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Self {
            position: [x, y, z],
            tex_coords: [u, v],
        }
    }
}

struct Mesh {
    vertex_buffer: VertexBufferAny,
    indices: Option<IndexBufferAny>,
}

struct OglDrawData {
    mesh: Rc<Mesh>,
    texture: Rc<Texture2d>,
}

impl DrawData for OglDrawData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct OglRenderer {
    window: Window,
    display: Display<WindowSurface>,
    program: Program,
    meshes: HashMap<String, Weak<Mesh>>,
    textures: HashMap<String, Weak<Texture2d>>,
}

impl OglRenderer {
    pub fn new(event_loop: &EventLoop<()>, window_builder: WindowBuilder) -> Self {
        let display_builder = glutin_winit::DisplayBuilder::new()
            .with_window_builder(Some(window_builder));
        let config_template_builder = ConfigTemplateBuilder::new();
        let (window, gl_config) = display_builder.build(event_loop, config_template_builder, |mut configs| {
            configs.next().unwrap()
        }).unwrap();
        let window = window.unwrap();
        let (width, height) = window.inner_size().into();
        let attrs = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
            .build(
                window.raw_window_handle(),
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap()
            );

        let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };
        
        let context_attributes = glutin::context::ContextAttributesBuilder::new()
            .build(Some(window.raw_window_handle()));
        let current_context = Some(unsafe {
            gl_config.display().create_context(&gl_config, &context_attributes).expect("failed to create context")
        }).unwrap().make_current(&surface).unwrap();

        // Vsync
        // surface.set_swap_interval(&current_context, glutin::surface::SwapInterval::Wait(unsafe { NonZeroU32::new_unchecked(1) })).unwrap();

        let display = Display::from_context_surface(current_context, surface).unwrap();

        let program = Program::from_source(
            &display,
            fs::read_to_string("res/shaders/vertex.glsl").unwrap().as_str(),
            fs::read_to_string("res/shaders/fragment.glsl").unwrap().as_str(),
            None,
        ).unwrap();

        let mut meshes = HashMap::new();
        meshes.insert("internal::triangle".to_owned(), unsafe {
            let rc = Rc::new(gen_triangle(&display));
            let raw = Rc::into_raw(rc);
            Rc::increment_strong_count(raw);
            Weak::from_raw(raw)
        });
        meshes.insert("internal::square".to_owned(), unsafe {
            let rc = Rc::new(gen_square(&display));
            let raw = Rc::into_raw(rc);
            Rc::increment_strong_count(raw);
            Weak::from_raw(raw)
        });
        meshes.insert("internal::cube".to_owned(), unsafe {
            let rc = Rc::new(gen_cube(&display));
            let raw = Rc::into_raw(rc);
            Rc::increment_strong_count(raw);
            Weak::from_raw(raw)
        });

        Self {
            window,
            display,
            program,
            meshes,
            textures: HashMap::new(),
        }
    }

    fn load_mesh(&mut self, mesh_name: &str) -> Rc<Mesh> {
        if let Some(i) = self.meshes.get(mesh_name) {
            if let Some(strong) = i.upgrade() {
                return strong;
            }
        }

        todo!()
    }

    fn load_texture(&mut self, texture_name: &str) -> Rc<Texture2d> {
        if let Some(i) = self.textures.get(texture_name) {
            if let Some(strong) = i.upgrade() {
                return strong;
            }
        }

        let image = image::io::Reader::open(texture_name)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = Rc::new(Texture2d::new(&self.display, image).unwrap());

        self.textures.insert(texture_name.to_owned(), Rc::downgrade(&texture));

        texture
    }
}

impl Renderer for OglRenderer {
    fn get_window(&self) -> &Window {
        &self.window
    }
    
    fn render(&mut self, world: &mut brood::World<crate::components::Registry, crate::resources::Resources>) {
        world.run_system(self);
    }
    
    fn load(&mut self, mesh_name: &str, texture_name: &str) -> DrawComponent {
        DrawComponent { inner: Box::new(OglDrawData {
            mesh: self.load_mesh(mesh_name),
            texture: self.load_texture(texture_name),
        })}
    }
}

impl System for OglRenderer {
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

        for result!(transform, draw) in query_result.iter {
            let ogl_draw = draw.inner.as_any().downcast_ref::<OglDrawData>().unwrap();
            match &ogl_draw.mesh.indices {
                Some(i) => {
                    target
                        .draw(
                            &ogl_draw.mesh.vertex_buffer,
                            i,
                            &self.program,
                            &uniform! {
                                matrix: camera.transform(transform.inner()),
                                tex: ogl_draw.texture.as_ref(),
                            },
                            &draw_parameters,
                        )
                        .unwrap();
                }
                None => {
                    target
                        .draw(
                            &ogl_draw.mesh.vertex_buffer,
                            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                            &self.program,
                            &uniform! {
                                matrix: camera.transform(transform.inner()),
                                tex: ogl_draw.texture.as_ref(),
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

fn gen_triangle(display: &Display<WindowSurface>) -> Mesh {
    let triangle_verts = vec![
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0),
        Vertex::new(0.0, 0.5, 0.0, 1.0, 1.0),
    ];

    Mesh {
        vertex_buffer: glium::VertexBuffer::new(display, &triangle_verts)
            .unwrap()
            .into(),
        indices: None,
    }
}

fn gen_square(display: &Display<WindowSurface>) -> Mesh {
    let square_verts = vec![
        Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0),
        Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0),
    ];

    let square_indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];

    Mesh {
        vertex_buffer: glium::VertexBuffer::new(display, &square_verts)
            .unwrap()
            .into(),
        indices: Some(
            glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &square_indices,
            )
            .unwrap()
            .into(),
        ),
    }
}

fn gen_cube(display: &Display<WindowSurface>) -> Mesh {
    let cube_verts = vec![
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 0.0),
        Vertex::new(0.5, -0.5, -0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 1.0),
        Vertex::new(-0.5, 0.5, 0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
    ];

    Mesh {
        vertex_buffer: glium::VertexBuffer::new(display, &cube_verts)
            .unwrap()
            .into(),
        indices: None,
    }
}
