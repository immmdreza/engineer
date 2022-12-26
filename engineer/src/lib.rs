extern crate engineer_derive;

pub use engineer_derive::*;

pub trait Engineer {
    const NORMAL_FIELDS: usize;
    const OPTIONAL_FIELDS: usize;

    type Target;
}
