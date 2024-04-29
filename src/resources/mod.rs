use brood::Resources;

use self::{camera::CameraResource, input::InputResource, time::TimerResource};

pub mod camera;
pub mod input;
pub mod time;

pub struct ExitResource(pub bool);

pub type Resources = Resources!(CameraResource, TimerResource, InputResource, ExitResource);
