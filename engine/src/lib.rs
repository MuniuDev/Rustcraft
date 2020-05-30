mod config;
mod io;
mod system;
mod util;
mod rendering;
pub mod model;
pub mod task;
pub mod core;
pub mod engine;
pub mod app;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}