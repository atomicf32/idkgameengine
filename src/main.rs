pub mod components;
pub mod render;
pub mod resources;
pub mod systems;

use std::time::Duration;

use brood::{entity, resources, schedule, system::schedule::task, World};
use glam::{Mat4, Quat, Vec3};

use resources::{camera::CameraResource, input::InputResource, time::TimerResource, ExitResource, Resources};
use systems::{camera_system::CameraSystem, close_system::CloseSystem, spin_system::SpinCube};
use winit::{
    event::{Event, WindowEvent}, event_loop::EventLoopBuilder, window::{CursorGrabMode, WindowBuilder}
};

use components::{transform::TransformComponent, Registry};
use render::{ogl_renderer::OglRenderer, *};

fn main() {
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("Event loop didn't build");
    let window_builder = WindowBuilder::new()
        .with_title("onlyopps (Every opp shot) (Opp Stoppa - YBN Nahmir) - Justin Jazzy Not");

    let mut renderer: Box<dyn Renderer> = Box::new(OglRenderer::new(&event_loop, window_builder));

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    renderer.get_window().set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    renderer.get_window().set_cursor_visible(false);
    renderer.get_window().set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| renderer.get_window().set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();

    let mut world = World::<Registry, Resources>::with_resources(resources!(
        CameraResource::new(
            60_f32.to_radians(),
            renderer.get_window().inner_size().width as f32 / renderer.get_window().inner_size().height as f32
        ),
        TimerResource::new(Duration::from_secs_f32(1 as f32 / 10 as f32)),
        InputResource::new(renderer.get_window()),
        ExitResource(false),
    ));

    world.insert(entity!(
        TransformComponent::from_mat4(Mat4::from_rotation_translation(
            Quat::from_euler(
                glam::EulerRot::XYZ,
                40_f32.to_radians(),
                0.0,
                40_f32.to_radians()
            ),
            Vec3::new(0.0, 0.0, 5.0)
        )),
        renderer.load("internal::cube", "res/textures/container.jpg")
    ));

    world.insert(entity!(
        TransformComponent::from_mat4(Mat4::from_rotation_translation(
            Quat::from_euler(
                glam::EulerRot::XYZ,
                40_f32.to_radians(),
                0.0,
                40_f32.to_radians()
            ),
            Vec3::new(1.0, -2.0, 7.0)
        )),
        renderer.load("internal::cube", "res/textures/container.jpg")
    ));

    world.insert(entity!(
        TransformComponent::from_mat4(Mat4::from_rotation_translation(
            Quat::from_euler(
                glam::EulerRot::XYZ,
                40_f32.to_radians(),
                0.0,
                40_f32.to_radians()
            ),
            Vec3::new(-1.0, -3.0, 2.0)
        )),
        renderer.load("internal::cube", "res/textures/container.jpg")
    ));

    let mut schedule = schedule!(
        task::System(SpinCube),
        task::System(CameraSystem),
        task::System(CloseSystem),
    );

    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::DeviceEvent { event, .. } => {
                world.get_mut::<InputResource, _>().device_event(&event);
            }
            Event::WindowEvent { event, .. } => {
                world.get_mut::<InputResource, _>().window_event(&event);

                match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::Resized(size) => world
                        .get_mut::<CameraResource, _>()
                        .resize(size.width as f32 / size.height as f32),
                    _ => (),
                }
            }
            Event::AboutToWait => {
                world.run_schedule(&mut schedule);
                renderer.render(&mut world);
                world.get_mut::<TimerResource, _>().tick();
                world.get_mut::<InputResource, _>().tick();
                if world.get::<ExitResource, _>().0 {
                    window_target.exit();
                }
            }
            _ => (),
        }
    });
}
