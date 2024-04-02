pub mod components;
pub mod render;
pub mod systems;
pub mod resources;

use std::time::Duration;

use brood::{entity, resources, schedule, system::schedule::task, World};
use glam::{Mat4, Quat, Vec3};
use glium::backend::glutin::SimpleWindowBuilder;
use resources::{camera::CameraResource, time::TimerResource};
use systems::MoveCamera;
use winit::{event::{Event, WindowEvent}, event_loop::EventLoopBuilder};

use render::{mesh_manager::*, renderer::Renderer, shader_manager::*, *};
use components::{render::RenderComponent, transform::TransformComponent, Registry};

fn main() {
    let event_loop = EventLoopBuilder::new().build().expect("Event loop didn't build");
    let (window, display) = SimpleWindowBuilder::new()
        .with_title("onlyopps (Every opp shot) (Opp Stoppa - YBN Nahmir) - Justin Jazzy Not")
        .build(&event_loop);

    let mut renderer = Renderer::new(display);

    let mut world = World::<Registry, _>::with_resources(resources!(
        CameraResource::new(60_f32.to_radians(), window.inner_size().width as f32 / window.inner_size().height as f32),
        TimerResource::new(Duration::from_secs_f32(1 as f32 / 10 as f32)),
    ));

    world.insert(entity!(
        TransformComponent::from_mat4(Mat4::from_rotation_translation(Quat::from_euler(glam::EulerRot::XYZ, 40_f32.to_radians(), 0.0, 40_f32.to_radians()), Vec3::new(0.0, 0.0, 5.0))),
        RenderComponent::from_renderer(&mut renderer, CUBE_ID, &DEFAULT_SHADERS)
    ));

    let mut schedule = schedule!(task::System(MoveCamera));

    let _ = event_loop.run(move |event, window_target| {
        window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);
        match event {
            Event::DeviceEvent { device_id, event } => {

            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::Resized(size) => world.get_mut::<CameraResource, _>().resize(size.width as f32 / size.height as f32),
                _ => (),
            }
            Event::AboutToWait => {
                world.get_mut::<TimerResource, _>().tick();
                world.run_schedule(&mut schedule);
                world.run_system(&mut renderer);
            }
            _ => (),
        }
    });
}
