use brood::{query::{filter, result}, system::System, Views};
use glam::{Mat4, Quat, Vec3};
use winit::keyboard::KeyCode;

use crate::resources::{camera::CameraResource, input::InputResource, time::TimerResource};

const CAMERA_SPEED: f32 = 5.0;
const CAMERA_SENS: f32 = 0.001;

pub struct CameraSystem;

impl System for CameraSystem {
	type Filter = filter::None;
	type Views<'a> = Views!();
	type ResourceViews<'a> = Views!(&'a mut CameraResource, &'a InputResource, &'a TimerResource);
	type EntryViews<'a> = Views!();

	fn run<'a, R, S, I, E>(
		&mut self,
		query_result: brood::query::Result<'a, R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>,
	) where
		R: brood::registry::ContainsViews<'a, Self::EntryViews<'a>, E>,
		I: Iterator<Item = Self::Views<'a>>
	{
		let result!(camera, input, timer) = query_result.resources;

		camera.transform_local(Mat4::from_quat(Quat::from_euler(glam::EulerRot::XYZ, input.get_mouse_delta().y * CAMERA_SENS, input.get_mouse_delta().x * CAMERA_SENS, 0.0)));

		if input.key_pressed(KeyCode::KeyW) {
			camera.transform_local(Mat4::from_translation(Vec3::new(0.0, 0.0, CAMERA_SPEED * timer.get_dt())))
		}
		if input.key_pressed(KeyCode::KeyA) {
			camera.transform_local(Mat4::from_translation(Vec3::new(-CAMERA_SPEED * timer.get_dt(), 0.0, 0.0)))
		}
		if input.key_pressed(KeyCode::KeyS) {
			camera.transform_local(Mat4::from_translation(Vec3::new(0.0, 0.0, -CAMERA_SPEED * timer.get_dt())))
		}
		if input.key_pressed(KeyCode::KeyD) {
			camera.transform_local(Mat4::from_translation(Vec3::new(CAMERA_SPEED * timer.get_dt(), 0.0, 0.0)))
		}
		if input.key_pressed(KeyCode::KeyQ) {
			camera.transform_local(Mat4::from_translation(Vec3::new(0.0, CAMERA_SPEED * timer.get_dt(), 0.0)))
		}
		if input.key_pressed(KeyCode::KeyE) {
			camera.transform_local(Mat4::from_translation(Vec3::new(0.0, -CAMERA_SPEED * timer.get_dt(), 0.0)))
		}
	}
}