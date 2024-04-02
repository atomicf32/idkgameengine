use std::collections::LinkedList;

use winit::event::{DeviceEvent, DeviceId};

pub struct InputCollector(LinkedList<(DeviceId, DeviceEvent)>);

impl InputCollector {
	pub fn new() -> Self {
		Self(LinkedList::new())
	}

	pub fn push(&mut self, id: DeviceId, event: DeviceEvent) {
		self.0.push_back((id, event));
	}
}

pub struct InputResource(Vec<(DeviceId, DeviceEvent)>);

impl InputResource {
	
}
