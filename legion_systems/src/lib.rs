pub mod resource;
pub mod schedule;

mod system_fn;
mod system;

pub use bit_set;
pub use system::*;
pub use system_fn::*;

pub mod prelude {
    pub use crate::{
        bit_set::BitSet,
        resource::{ResourceSet, Resources},
        schedule::{Executor, Runnable, Schedulable, Schedule},
        System, SystemBuilder,
    };
}
