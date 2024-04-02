use brood::{query::filter, system::System, Views};

use crate::{resources::time::TimerResource, CameraResource};

pub struct MoveCamera;

impl System for MoveCamera {
	type Filter = filter::None;
	type Views<'a> = Views!();
	type ResourceViews<'a> = Views!(&'a mut CameraResource, &'a TimerResource);
	type EntryViews<'a> = Views!();

	fn run<'a, R, S, I, E>(
		&mut self,
		query_result: brood::query::Result<'a, R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>,
	) where
		R: brood::registry::ContainsViews<'a, Self::EntryViews<'a>, E>,
		I: Iterator<Item = Self::Views<'a>>
	{
		let (camera, (timer, _)) = query_result.resources;
		
	}
}