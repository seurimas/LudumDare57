use crate::prelude::*;

pub mod map;
pub use map::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_towns);
    }
}
