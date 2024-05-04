use std::f32::consts::PI;

use brood::{
    query::{filter, result},
    system::System,
    Views,
};
use glam::{Mat3A, Quat, Vec3A};
use winit::keyboard::KeyCode;

use crate::resources::{camera::CameraResource, input::InputResource, time::TimerResource};

const CAMERA_SPEED: f32 = 5.0;
const CAMERA_SENS: f32 = 0.001;

#[derive(Default)]
pub struct CameraSystem;

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

        let (mut y, mut x, _) = camera.rotation.to_euler(glam::EulerRot::YXZ);

        y = input.get_mouse_delta().x * CAMERA_SENS + y;
        x = (input.get_mouse_delta().y * CAMERA_SENS + x).clamp(-PI/2.0, PI/2.0);

        camera.rotation = Quat::from_euler(glam::EulerRot::YXZ, y, x, 0.0);

        let mut raw_dir = Vec3A::ZERO;
        
        if input.key_pressed(KeyCode::KeyD) {
            raw_dir.x += 1.0;
        }
        if input.key_pressed(KeyCode::KeyE) {
            raw_dir.y += 1.0;
        }
        if input.key_pressed(KeyCode::KeyW) {
            raw_dir.z += 1.0;
        }

        if input.key_pressed(KeyCode::KeyA) {
            raw_dir.x -= 1.0;
        }
        if input.key_pressed(KeyCode::KeyQ) {
            raw_dir.y -= 1.0;
        }
        if input.key_pressed(KeyCode::KeyS) {
            raw_dir.z -= 1.0;
        }

        raw_dir = Mat3A::from_quat(camera.rotation) * raw_dir;
        
        camera.translation += raw_dir.normalize_or_zero() * CAMERA_SPEED * timer.get_dt_f32();
    }
}
