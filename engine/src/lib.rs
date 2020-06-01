#[macro_use]
extern crate static_assertions;
#[macro_use]
extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;

// MOD STATEMENTS
mod features;
mod io;
mod system;
mod util;

// PUB MOD STATEMENTS
pub mod core;
pub mod model;
pub mod task;
pub mod engine;

// FEATURES STATEMENTS
#[cfg(feature = "graphics")]
pub mod rendering;

// USE STATEMENTS
pub use crate::core::*;
pub use features::Features;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}