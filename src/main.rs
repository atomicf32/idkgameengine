use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{event::{Event, WindowEvent}, event_loop::EventLoopBuilder};

fn main() {
    let event_loop = EventLoopBuilder::new().build().expect("Event loop didn't build");
    let (_window, display) = SimpleWindowBuilder::new()
        .with_title("ogl")
        .build(&event_loop);

    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            }
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.1, 0.4, 1.0);
        target.finish().unwrap();
    });
}
