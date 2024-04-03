use std::collections::HashSet;

use winit::{dpi::PhysicalPosition, event::{DeviceEvent, MouseButton, MouseScrollDelta, WindowEvent}, keyboard::PhysicalKey, window::Window};

pub struct InputResource {
	focused: bool,
	cursor_pos: PhysicalPosition<f32>,
	mouse_wheel_delta: PhysicalPosition<f32>,
	mouse_delta: PhysicalPosition<f32>,
	pressed_keys: HashSet<PhysicalKey>,
	pressed_mouse_buttons: HashSet<MouseButton>
}

impl InputResource {
	pub fn new(window: &Window) -> Self {
		Self {
			focused: window.has_focus(),
			cursor_pos: PhysicalPosition { x: 0.0, y: 0.0 },
			mouse_wheel_delta: PhysicalPosition { x: 0.0, y: 0.0 },
			mouse_delta: PhysicalPosition { x: 0.0, y: 0.0 },
			pressed_keys: HashSet::new(),
			pressed_mouse_buttons: HashSet::new(),
		}
	}

	pub fn tick(&mut self) {
		self.mouse_wheel_delta = PhysicalPosition { x: 0.0, y: 0.0 };
		self.mouse_delta = PhysicalPosition { x: 0.0, y: 0.0 };
	}

	pub fn device_event(&mut self, event: &DeviceEvent) {
		if let DeviceEvent::MouseMotion { delta } = event {
			self.mouse_delta.x += delta.0 as f32;
			self.mouse_delta.y += delta.1 as f32;
		}
	}

	pub fn window_event(&mut self, event: &WindowEvent) {
		match event {
			WindowEvent::Focused(is_focused) => self.focused = *is_focused,
			WindowEvent::KeyboardInput { event, .. } => {
				match event.state {
					winit::event::ElementState::Pressed => { self.pressed_keys.insert(event.physical_key); }
					winit::event::ElementState::Released => { self.pressed_keys.remove(&event.physical_key); }
				}
			}
			WindowEvent::CursorMoved { position, .. } => { self.cursor_pos = physical_pos_cast(position) }
			WindowEvent::MouseWheel { delta, .. } => {
				if let MouseScrollDelta::PixelDelta(d) = delta {
					self.mouse_wheel_delta.x += d.x as f32;
					self.mouse_wheel_delta.y += d.y as f32;
				}
			}
			WindowEvent::MouseInput { state, button, .. } => {
				match state {
						winit::event::ElementState::Pressed => { self.pressed_mouse_buttons.insert(*button); }
						winit::event::ElementState::Released => { self.pressed_mouse_buttons.remove(button); }
					}
			}
			_ => {}
		}
	}

	pub fn get_mouse_delta() {

	}
}

fn physical_pos_cast(physical_pos: &PhysicalPosition<f64>) -> PhysicalPosition<f32> {
	PhysicalPosition { x: physical_pos.x as f32, y: physical_pos.y as f32 }
}