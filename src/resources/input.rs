use bitvec::prelude::*;
use brood::{query::filter, result, system::System, Views};
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceEvent, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct InputResource {
    focused: bool,
    cursor_pos: PhysicalPosition<f32>,
    scroll_delta: PhysicalPosition<f32>,
    mouse_delta: PhysicalPosition<f32>,
    pressed_keys: BitArray<[usize; 4], Lsb0>,
    pressed_mouse_buttons: BitArray<[u8; 1], Lsb0>,
}

impl InputResource {
    pub fn new(focused: bool) -> Self {
        Self {
            focused,
            cursor_pos: PhysicalPosition { x: 0.0, y: 0.0 },
            scroll_delta: PhysicalPosition { x: 0.0, y: 0.0 },
            mouse_delta: PhysicalPosition { x: 0.0, y: 0.0 },
            pressed_keys: BitArray::new([0; 4]),
            pressed_mouse_buttons: BitArray::new([0]),
        }
    }

    pub fn device_event(&mut self, event: &DeviceEvent) {
        if self.focused {
            if let DeviceEvent::MouseMotion { delta } = event {
                self.mouse_delta.x += delta.0 as f32;
                self.mouse_delta.y += delta.1 as f32;
            }
        }
    }

    pub fn window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Focused(is_focused) => self.focused = *is_focused,
            WindowEvent::KeyboardInput { event, .. } => match event.state {
                winit::event::ElementState::Pressed => {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        self.pressed_keys.set(code as usize, true);
                    }
                }
                winit::event::ElementState::Released => {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        self.pressed_keys.set(code as usize, false);
                    }
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = physical_pos_cast(position)
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::PixelDelta(d) = delta {
                    self.scroll_delta.x += d.x as f32;
                    self.scroll_delta.y += d.y as f32;
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                winit::event::ElementState::Pressed => {
                    self.pressed_mouse_buttons
                        .set(mouse_button_to_usize(button), true);
                }
                winit::event::ElementState::Released => {
                    self.pressed_mouse_buttons
                        .set(mouse_button_to_usize(button), false);
                }
            },
            _ => {}
        }
    }

    pub fn get_mouse_delta(&self) -> PhysicalPosition<f32> {
        self.mouse_delta
    }

    pub fn get_scroll_delta(&self) -> PhysicalPosition<f32> {
        self.scroll_delta
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys[key as usize]
    }
}

fn physical_pos_cast(physical_pos: &PhysicalPosition<f64>) -> PhysicalPosition<f32> {
    PhysicalPosition {
        x: physical_pos.x as f32,
        y: physical_pos.y as f32,
    }
}

fn mouse_button_to_usize(button: &MouseButton) -> usize {
    match button {
        MouseButton::Left => 1,
        MouseButton::Right => 2,
        MouseButton::Middle => 3,
        MouseButton::Back => 4,
        MouseButton::Forward => 5,
        MouseButton::Other(_) => 6,
    }
}

pub struct TickInput;

impl System for TickInput {
    type Filter = filter::None;
    type Views<'a> = Views!();
    type ResourceViews<'a> = Views!(&'a mut InputResource);
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
        R: brood::registry::ContainsViews<'a, Self::EntryViews<'a>, E>,
        I: Iterator<Item = Self::Views<'a>>,
    {
        let result!(input) = query_result.resources;

        input.scroll_delta = PhysicalPosition { x: 0.0, y: 0.0 };
        input.mouse_delta = PhysicalPosition { x: 0.0, y: 0.0 };
    }
}
