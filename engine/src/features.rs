macro_rules! is_feature_enabled {
    ($name:expr) => (
        (||{
        #[allow(unused_mut, unused_assignments)]
        let mut val = false;
        #[cfg(feature = $name)]
        { val = true; }
        val
        })();
    );
}

#[derive(Debug)]
pub struct Features {
    graphics: bool,
    audio: bool,
    networking: bool
}

impl Features {
    pub fn enabled() -> Self {
        return Features {
            graphics: is_feature_enabled!("graphics"),
            audio: is_feature_enabled!("audio"),
            networking: is_feature_enabled!("networking"),
        };
    }
}