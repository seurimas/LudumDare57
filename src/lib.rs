pub mod game;
pub mod game_state;
pub mod prelude;
pub const PIXELS_PER_METER: f32 = 32.0;
pub const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;

pub use derive_new::new;
