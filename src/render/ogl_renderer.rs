use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    num::NonZeroU32,
    path::Path,
    rc::{Rc, Weak},
};

use crate::{
    components::{draw::DrawComponent, transform::TransformComponent},
    resources::{camera::CameraResource, SunResource},
    DrawData, DrawDescriptor, Mesh, Renderer,
};
use brood::{query::filter, registry, result, system::System, Views};
use glium::{
    glutin::surface::WindowSurface, implement_vertex, texture::RawImage2d, uniform, Display,
    DrawParameters, Program, Surface, Texture2d,
};
use glium::{index::IndexBufferAny, vertex::VertexBufferAny};
use glutin::{
    config::ConfigTemplate,
    context::NotCurrentGlContext,
    display::{GetGlDisplay, GlDisplay},
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::window::Window;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, normal_x: f32, normal_y: f32, normal_z: f32, u: f32, v: f32) -> Self {
        Self {
            position: [x, y, z],
            normal: [normal_x, normal_y, normal_z],
            tex_coords: [u, v],
        }
    }
}

struct OglMesh {
    vertex_buffer: VertexBufferAny,
    indices: Option<IndexBufferAny>,
}

struct OglDrawData {
    mesh: Rc<OglMesh>,
    texture: Rc<Texture2d>,
}

impl DrawData for OglDrawData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct OglRenderer {
    display: Display<WindowSurface>,
    program: Program,
    meshes: HashMap<Mesh, Weak<OglMesh>>,
    textures: HashMap<Cow<'static, Path>, Weak<Texture2d>>,
}

impl OglRenderer {
    pub fn new(window: &Window) -> Self {
        #[cfg(target_os = "windows")]
        let glutin_display = unsafe {
            glutin::display::Display::new(
                window.raw_display_handle(),
                glutin::display::DisplayApiPreference::EglThenWgl(Some(window.raw_window_handle())),
            )
        }
        .unwrap();

        let gl_config = unsafe { glutin_display.find_configs(ConfigTemplate::default()) }
            .unwrap()
            .next()
            .unwrap();

        let (width, height) = window.inner_size().into();
        let attrs =
            glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
                .build(
                    window.raw_window_handle(),
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                );

        let surface = unsafe {
            glutin_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let context_attributes = glutin::context::ContextAttributesBuilder::new()
            .build(Some(window.raw_window_handle()));
        let current_context = Some(unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .expect("failed to create context")
        })
        .unwrap()
        .make_current(&surface)
        .unwrap();

        // Vsync
        // surface.set_swap_interval(&current_context, glutin::surface::SwapInterval::Wait(unsafe { NonZeroU32::new_unchecked(1) })).unwrap();

        let display = Display::from_context_surface(current_context, surface).unwrap();

        let program = Program::from_source(
            &display,
            fs::read_to_string("res/shaders/vertex.glsl")
                .unwrap()
                .as_str(),
            fs::read_to_string("res/shaders/fragment.glsl")
                .unwrap()
                .as_str(),
            None,
        )
        .unwrap();

        let mut meshes = HashMap::new();
        meshes.insert(Mesh::Triangle, unsafe {
            let rc = Rc::new(gen_triangle(&display));
            let raw = Rc::into_raw(rc);
            Rc::increment_strong_count(raw);
            Weak::from_raw(raw)
        });
        meshes.insert(Mesh::Square, unsafe {
            let rc = Rc::new(gen_square(&display));
            let raw = Rc::into_raw(rc);
            Rc::increment_strong_count(raw);
            Weak::from_raw(raw)
        });
        meshes.insert(Mesh::Cube, unsafe {
            let rc = Rc::new(gen_cube(&display));
            let raw = Rc::into_raw(rc);
            Rc::increment_strong_count(raw);
            Weak::from_raw(raw)
        });

        Self {
            display,
            program,
            meshes,
            textures: HashMap::new(),
        }
    }

    fn load_mesh(&mut self, mesh_name: &Mesh) -> Rc<OglMesh> {
        if let Some(i) = self.meshes.get(mesh_name) {
            if let Some(strong) = i.upgrade() {
                return strong;
            }
        }

        let mesh = match mesh_name {
            Mesh::Gltf(path) => {
                let gltf = easy_gltf::load(path).unwrap();
                let scene = &gltf[0];
                let model = &scene.models[0];

                let mut vertices = Vec::with_capacity(model.vertices().len());

                for i in model.vertices() {
                    vertices.push(Vertex::new(
                        i.position.x,
                        i.position.y,
                        i.position.z,
                        i.normal.x,
                        i.normal.y,
                        i.normal.z,
                        i.tex_coords.x,
                        i.tex_coords.y,
                    ))
                }

                let mut indices = None;

                if let Some(i) = model.indices() {
                    indices = Some(
                        glium::IndexBuffer::new(
                            &self.display,
                            glium::index::PrimitiveType::TrianglesList,
                            &i,
                        )
                        .unwrap()
                        .into(),
                    );
                }

                Rc::new(OglMesh {
                    vertex_buffer: glium::VertexBuffer::new(&self.display, &vertices)
                        .unwrap()
                        .into(),
                    indices,
                })
            }
            _ => panic!("Internal mesh not in hashmap?"),
        };

        self.meshes.insert(mesh_name.clone(), Rc::downgrade(&mesh));

        mesh
    }

    fn load_texture(&mut self, texture_name: &Cow<'static, Path>) -> Rc<Texture2d> {
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

        self.textures
            .insert(texture_name.clone(), Rc::downgrade(&texture));

        texture
    }
}

impl Renderer for OglRenderer {
    fn render(
        &mut self,
        world: &mut brood::World<crate::components::Registry, crate::resources::Resources>,
    ) {
        world.run_system(self);
    }

    fn load(&mut self, descriptor: &DrawDescriptor) -> DrawComponent {
        DrawComponent {
            inner: Box::new(OglDrawData {
                mesh: self.load_mesh(&descriptor.mesh),
                texture: self.load_texture(&descriptor.texture),
            }),
        }
    }
}

impl System for OglRenderer {
    type Filter = filter::None;
    type Views<'a> = Views!(&'a TransformComponent, &'a DrawComponent);
    type ResourceViews<'a> = Views!(&'a CameraResource, &'a SunResource);
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

        let result!(camera, sun) = query_result.resources;

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
                                camera_mat: camera.get_mat_array(),
                                model_mat: transform.get_mat_array(),
                                tex: ogl_draw.texture.as_ref(),
                                light_pos: sun.0.to_array(),
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
                                camera_mat: camera.get_mat_array(),
                                model_mat: transform.get_mat_array(),
                                tex: ogl_draw.texture.as_ref(),
                                light_pos: sun.0.to_array(),
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

fn gen_triangle(display: &Display<WindowSurface>) -> OglMesh {
    let triangle_verts = vec![
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0),
        Vertex::new(0.0, 0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
    ];

    OglMesh {
        vertex_buffer: glium::VertexBuffer::new(display, &triangle_verts)
            .unwrap()
            .into(),
        indices: None,
    }
}

fn gen_square(display: &Display<WindowSurface>) -> OglMesh {
    let square_verts = vec![
        Vertex::new(0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
        Vertex::new(0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0),
    ];

    let square_indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];

    OglMesh {
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

fn gen_cube(display: &Display<WindowSurface>) -> OglMesh {
    let cube_verts = vec![
        Vertex::new(-0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 0.0, 0.0),
        Vertex::new( 0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 1.0, 0.0),
        Vertex::new( 0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 1.0, 1.0),
        Vertex::new( 0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 1.0, 1.0),
        Vertex::new(-0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5,  0.5,  0.0,  0.0,  1.0, 0.0, 0.0),
        Vertex::new( 0.5, -0.5,  0.5,  0.0,  0.0,  1.0, 1.0, 0.0),
        Vertex::new( 0.5,  0.5,  0.5,  0.0,  0.0,  1.0, 1.0, 1.0),
        Vertex::new( 0.5,  0.5,  0.5,  0.0,  0.0,  1.0, 1.0, 1.0),
        Vertex::new(-0.5,  0.5,  0.5,  0.0,  0.0,  1.0, 0.0, 1.0),
        Vertex::new(-0.5, -0.5,  0.5,  0.0,  0.0,  1.0, 0.0, 0.0),
        Vertex::new(-0.5,  0.5,  0.5, -1.0,  0.0,  0.0, 1.0, 0.0),
        Vertex::new(-0.5,  0.5, -0.5, -1.0,  0.0,  0.0, 1.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, -1.0,  0.0,  0.0, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, -1.0,  0.0,  0.0, 0.0, 1.0),
        Vertex::new(-0.5, -0.5,  0.5, -1.0,  0.0,  0.0, 0.0, 0.0),
        Vertex::new(-0.5,  0.5,  0.5, -1.0,  0.0,  0.0, 1.0, 0.0),
        Vertex::new( 0.5,  0.5,  0.5,  1.0,  0.0,  0.0, 1.0, 0.0),
        Vertex::new( 0.5,  0.5, -0.5,  1.0,  0.0,  0.0, 1.0, 1.0),
        Vertex::new( 0.5, -0.5, -0.5,  1.0,  0.0,  0.0, 0.0, 1.0),
        Vertex::new( 0.5, -0.5, -0.5,  1.0,  0.0,  0.0, 0.0, 1.0),
        Vertex::new( 0.5, -0.5,  0.5,  1.0,  0.0,  0.0, 0.0, 0.0),
        Vertex::new( 0.5,  0.5,  0.5,  1.0,  0.0,  0.0, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5,  0.0, -1.0,  0.0, 0.0, 1.0),
        Vertex::new( 0.5, -0.5, -0.5,  0.0, -1.0,  0.0, 1.0, 1.0),
        Vertex::new( 0.5, -0.5,  0.5,  0.0, -1.0,  0.0, 1.0, 0.0),
        Vertex::new( 0.5, -0.5,  0.5,  0.0, -1.0,  0.0, 1.0, 0.0),
        Vertex::new(-0.5, -0.5,  0.5,  0.0, -1.0,  0.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5,  0.0, -1.0,  0.0, 0.0, 1.0),
        Vertex::new(-0.5,  0.5, -0.5,  0.0,  1.0,  0.0, 0.0, 1.0),
        Vertex::new( 0.5,  0.5, -0.5,  0.0,  1.0,  0.0, 1.0, 1.0),
        Vertex::new( 0.5,  0.5,  0.5,  0.0,  1.0,  0.0, 1.0, 0.0),
        Vertex::new( 0.5,  0.5,  0.5,  0.0,  1.0,  0.0, 1.0, 0.0),
        Vertex::new(-0.5,  0.5,  0.5,  0.0,  1.0,  0.0, 0.0, 0.0),
        Vertex::new(-0.5,  0.5, -0.5,  0.0,  1.0,  0.0, 0.0, 1.0),
    ];

    OglMesh {
        vertex_buffer: glium::VertexBuffer::new(display, &cube_verts)
            .unwrap()
            .into(),
        indices: None,
    }
}
