mod config;
mod io;
mod system;
mod util;
pub mod model;
pub mod task;
pub mod core;
pub mod engine;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}