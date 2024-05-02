pub mod components;
pub mod render;
pub mod resources;
pub mod systems;

use std::{path::Path, time::Duration};

use brood::{entity, resources, schedule, system::schedule::task, World};
use glam::{Mat4, Quat, Vec3};

use resources::{
    camera::CameraResource,
    input::{InputResource, TickInput},
    time::{TickTimer, TimerResource},
    ExitResource, Resources,
};
use systems::{camera_system::CameraSystem, close_system::CloseSystem, spin_system::SpinCube};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopBuilder,
    window::{CursorGrabMode, WindowAttributes},
};

use components::{transform::TransformComponent, Registry};
use render::{ogl_renderer::OglRenderer, *};

fn main() {
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("Event loop didn't build");
    let window = event_loop
        .create_window(WindowAttributes::default())
        .unwrap();

    let mut renderer: Box<dyn Renderer> = Box::new(OglRenderer::new(&window));

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    window.set_cursor_visible(false);
    window
        .set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
        .unwrap();

    let mut world = World::<Registry, Resources>::with_resources(resources!(
        CameraResource::new(
            60_f32.to_radians(),
            window.inner_size().width as f32 / window.inner_size().height as f32
        ),
        TimerResource::new(Duration::from_secs_f32(1 as f32 / 10 as f32)),
        InputResource::new(window.has_focus()),
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
        renderer.load(&DrawDescriptor {
            mesh: Mesh::Cube,
            texture: Path::new("res/textures/container.jpg").into(),
        })
    ));

    world.insert(entity!(
        TransformComponent::from_mat4(Mat4::from_scale_rotation_translation(
            Vec3::new(0.05, 0.05, 0.05),
            Quat::IDENTITY,
            Vec3::new(1.0, -2.0, 7.0)
        )),
        renderer.load(&DrawDescriptor {
            mesh: Mesh::Gltf(Path::new("res/gltf/teapot.gltf").into()),
            texture: Path::new("res/textures/container.jpg").into(),
        })
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
        renderer.load(&DrawDescriptor {
            mesh: Mesh::Cube,
            texture: Path::new("res/textures/container.jpg").into(),
        })
    ));

    let mut schedule = schedule!(
        task::System(SpinCube),
        task::System(CameraSystem),
        task::System(CloseSystem),
    );

    let mut tick_schedule = schedule!(task::System(TickTimer), task::System(TickInput));

    let _ = event_loop.run(move |event, window_target| match event {
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
                WindowEvent::RedrawRequested => {
                    world.run_schedule(&mut schedule);
                    renderer.render(&mut world);
                    world.run_schedule(&mut tick_schedule);
                    if world.get::<ExitResource, _>().0 {
                        window_target.exit();
                    }
                }
                _ => (),
            }
        }
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => (),
    });
}
