use brood::Resources;
use glam::Vec4;

use self::{camera::CameraResource, input::InputResource, time::TimerResource};

pub mod camera;
pub mod input;
pub mod time;

pub struct ExitResource(pub bool);

pub struct SunResource(pub Vec4);

pub type Resources = Resources!(CameraResource, TimerResource, InputResource, ExitResource, SunResource);
