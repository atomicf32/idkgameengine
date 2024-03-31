pub mod components;
pub mod render;

use brood::{entity, World};
use glam::{Mat4, Quat, Vec3};
use glium::backend::glutin::SimpleWindowBuilder;
use winit::{event::{Event, WindowEvent}, event_loop::EventLoopBuilder};

use render::{renderer::Renderer, *};
use components::{render::RenderComponent, transform::TransformComponent, Registry};

fn main() {
    let event_loop = EventLoopBuilder::new().build().expect("Event loop didn't build");
    let (_window, display) = SimpleWindowBuilder::new()
        .with_title("ogl")
        .build(&event_loop);

    let mut renderer = Renderer::new(display);

    let mut world = World::<Registry>::new();

    world.insert(entity!(
        TransformComponent::from_mat4(Mat4::from_rotation_translation(Quat::from_euler(glam::EulerRot::XYZ, 40_f32.to_radians(), 0.0, 40_f32.to_radians()), Vec3::new(0.0, 0.0, 5.0))),
        RenderComponent(renderer.get_mesh(CUBE_ID))
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
