extern crate engineer_derive;

pub use engineer_derive::*;

pub trait Builder<T> {
    fn done(self) -> T;
}

pub trait Engineer
where
    Self: Sized,
    Self::Builder: Builder<Self>,
{
    const NORMAL_FIELDS: usize;
    const OPTIONAL_FIELDS: usize;

    type Builder;
    type Params;

    fn builder(required: Self::Params) -> Self::Builder;

    fn build(required: Self::Params) -> Self {
        Self::builder(required).done()
    }

    fn build_default() -> Self
    where
        Self::Params: Default,
    {
        Self::build(Default::default())
    }
}
