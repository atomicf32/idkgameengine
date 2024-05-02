use brood::{query::filter, result, system::System, Views};
use winit::keyboard::KeyCode;

use crate::resources::{input::InputResource, ExitResource};

pub struct CloseSystem;

impl System for CloseSystem {
    type Filter = filter::None;
    type Views<'a> = Views!();
    type ResourceViews<'a> = Views!(&'a InputResource, &'a mut ExitResource);
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
        let result!(input, exit) = query_result.resources;

        if input.key_pressed(KeyCode::Escape) {
            exit.0 = true;
        }
    }
}
