use brood::{query::filter, result, system::System, Views};
use glam::{Mat4, Quat};

use crate::{components::transform::TransformComponent, resources::time::TimerResource, CameraResource};

pub struct MoveCamera;

impl System for MoveCamera {
	type Filter = filter::None;
	type Views<'a> = Views!(&'a mut TransformComponent);
	type ResourceViews<'a> = Views!(&'a mut CameraResource, &'a TimerResource);
	type EntryViews<'a> = Views!();

	fn run<'a, R, S, I, E>(
		&mut self,
		query_result: brood::query::Result<'a, R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>,
	) where
		R: brood::registry::ContainsViews<'a, Self::EntryViews<'a>, E>,
		I: Iterator<Item = Self::Views<'a>>
	{
		let result!(camera, timer) = query_result.resources;

		for result!(transform) in query_result.iter {
			transform.transform_local(Mat4::from_quat(Quat::from_euler(glam::EulerRot::XYZ, 45_f32.to_radians() * timer.get_dt(), 45_f32.to_radians() * timer.get_dt(), 0_f32)));
		}
	}
}