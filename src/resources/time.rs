use std::time::{Duration, Instant};

use brood::{query::filter, result, system::System, Views};

pub struct TimerResource {
    max_duration: Duration,
    start: Instant,
    previous: Instant,
    dt: Duration,
}

impl TimerResource {
    pub fn new(max_duration: Duration) -> Self {
        Self {
            max_duration,
            start: Instant::now(),
            previous: Instant::now(),
            dt: Duration::ZERO,
        }
    }

    pub fn get_runtime(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn get_dt(&self) -> f32 {
        self.dt.as_secs_f32()
    }
}

pub struct TickTimer;

impl System for TickTimer {
    type Filter = filter::None;
    type Views<'a> = Views!();
    type ResourceViews<'a> = Views!(&'a mut TimerResource);
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
        let result!(timer) = query_result.resources;

        let current = Instant::now();
        timer.dt = std::cmp::min(current - timer.previous, timer.max_duration);
        timer.previous = current;
    }
}
