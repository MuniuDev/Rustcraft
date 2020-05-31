// MOD STATEMENTS
mod features;
mod io;
mod system;
mod util;

// PUB MOD STATEMENTS
pub mod model;
pub mod task;
pub mod core;
pub mod engine;

// FEATURES STATEMENTS
#[cfg(feature = "graphics")]
pub mod rendering;

// USE STATEMENTS
pub use features::Features;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}