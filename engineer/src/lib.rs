extern crate engineer_derive;

pub use engineer_derive::*;

pub trait Builder<T> {
    fn done(self) -> T;
}

pub trait Engineer<B, P>
where
    Self: Sized,
    B: Builder<Self>,
{
    const NORMAL_FIELDS: usize;
    const OPTIONAL_FIELDS: usize;

    fn get_builder(required: P) -> B;
}
