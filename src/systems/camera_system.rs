use std::f32::consts::PI;

use brood::{
    query::{filter, result},
    system::System,
    Views,
};
use glam::Quat;
use winit::keyboard::KeyCode;

use crate::resources::{camera::CameraResource, input::InputResource, time::TimerResource};

const CAMERA_SPEED: f32 = 5.0;
const CAMERA_SENS: f32 = 0.001;

#[derive(Default)]
pub struct CameraSystem(f32, f32);

impl System for CameraSystem {
    type Filter = filter::None;
    type Views<'a> = Views!();
    type ResourceViews<'a> = Views!(&'a mut CameraResource, &'a InputResource, &'a TimerResource);
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
        let result!(camera, input, timer) = query_result.resources;

        self.0 = input.get_mouse_delta().x * CAMERA_SENS + self.0;
        self.1 = (input.get_mouse_delta().y * CAMERA_SENS + self.1).clamp(-PI/2.0, PI/2.0);

        camera.rotation = Quat::from_euler(glam::EulerRot::YXZ, self.0, self.1, 0.0);

        if input.key_pressed(KeyCode::KeyW) {
            camera.translation.x += self.0.sin() * CAMERA_SPEED * timer.get_dt_f32();
            camera.translation.z += self.0.cos() * CAMERA_SPEED * timer.get_dt_f32();
        }
        if input.key_pressed(KeyCode::KeyA) {
            camera.translation.x -= self.0.cos() * CAMERA_SPEED * timer.get_dt_f32();
            camera.translation.z -= self.0.sin() * CAMERA_SPEED * timer.get_dt_f32();
        }
        if input.key_pressed(KeyCode::KeyS) {
            camera.translation.x -= self.0.sin() * CAMERA_SPEED * timer.get_dt_f32();
            camera.translation.z -= self.0.cos() * CAMERA_SPEED * timer.get_dt_f32();
        }
        if input.key_pressed(KeyCode::KeyD) {
            camera.translation.x += self.0.cos() * CAMERA_SPEED * timer.get_dt_f32();
            camera.translation.z += self.0.sin() * CAMERA_SPEED * timer.get_dt_f32();
        }
        if input.key_pressed(KeyCode::KeyQ) {
            camera.translation.y -= CAMERA_SPEED * timer.get_dt_f32();
        }
        if input.key_pressed(KeyCode::KeyE) {
            camera.translation.y += CAMERA_SPEED * timer.get_dt_f32();
        }
    }
}
