mod app;
pub use app::App;

pub fn print_feat() {

    #[cfg(feature = "graphics")]
    println!("With Graphics = true");
    #[cfg(not(feature = "graphics"))]
    println!("With Graphics = false");
}