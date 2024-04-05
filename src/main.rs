pub mod components;
pub mod render;
pub mod resource_manager;
pub mod resources;
pub mod systems;

use std::{num::NonZeroU32, time::Duration};

use brood::{entity, resources, schedule, system::schedule::task, World};
use glam::{Mat4, Quat, Vec3};
use glium::Display;
use glutin::{config::ConfigTemplateBuilder, context::NotCurrentGlContext, display::{GetGlDisplay, GlDisplay}, surface::GlSurface};
use raw_window_handle::HasRawWindowHandle;
use resource_manager::ResourceManager;
use resources::{camera::CameraResource, input::InputResource, time::TimerResource, ExitResource};
use systems::{camera_system::CameraSystem, close_system::CloseSystem, spin_system::SpinCube};
use winit::{
    event::{Event, WindowEvent}, event_loop::EventLoopBuilder, window::{CursorGrabMode, WindowBuilder}
};

use components::{draw::DrawComponent, transform::TransformComponent, Registry};
use render::{renderer::Renderer, *};

fn main() {
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("Event loop didn't build");
    let (window, display) = {
        let window_builder = WindowBuilder::new()
            .with_title("onlyopps (Every opp shot) (Opp Stoppa - YBN Nahmir) - Justin Jazzy Not");
        let display_builder = glutin_winit::DisplayBuilder::new()
            .with_window_builder(Some(window_builder));
        let config_template_builder = ConfigTemplateBuilder::new();
        let (window, gl_config) = display_builder.build(&event_loop, config_template_builder, |mut configs| {
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
        surface.set_swap_interval(&current_context, glutin::surface::SwapInterval::Wait(unsafe { NonZeroU32::new_unchecked(1) })).unwrap();

        let display = Display::from_context_surface(current_context, surface).unwrap();

        (window, display)
    };

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    window.set_cursor_visible(false);
    window.set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();

    let mut renderer = Renderer::new(&display);
    let mut manager = ResourceManager::new(&display);

    let mut world = World::<Registry, _>::with_resources(resources!(
        CameraResource::new(
            60_f32.to_radians(),
            window.inner_size().width as f32 / window.inner_size().height as f32
        ),
        TimerResource::new(Duration::from_secs_f32(1 as f32 / 10 as f32)),
        InputResource::new(&window),
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
        DrawComponent::load(&mut manager, &Default::default())
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
                world.run_system(&mut renderer);
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
