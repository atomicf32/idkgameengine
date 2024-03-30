pub mod components;
pub mod render;

use brood::{entity, World};
use glium::backend::glutin::SimpleWindowBuilder;
use winit::{event::{Event, WindowEvent}, event_loop::EventLoopBuilder};

use render::renderer::Renderer;
use components::{mesh::RenderComponent, transform::TransformComponent, Registry};

fn main() {
    let event_loop = EventLoopBuilder::new().build().expect("Event loop didn't build");
    let (_window, display) = SimpleWindowBuilder::new()
        .with_title("ogl")
        .build(&event_loop);

    let mut renderer = Renderer::new(display);

    let mut world = World::<Registry>::new();

    world.insert(entity!(
        TransformComponent::from_position(0.0, 0.0, 5.0),
        RenderComponent::new(render::MeshType::Internal(render::InternalMesh::Cube))
    ));

    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            }
            _ => (),
        }

        world.run_system(&mut renderer);
    });
}
