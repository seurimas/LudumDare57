use crate::prelude::*;

mod map;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
pub use map::*;
mod town;
pub use town::*;
mod player;
pub use player::*;
mod town_ui;
pub use town_ui::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenTownEvent>()
            .add_systems(Update, (setup_town_uis, process_towns))
            .register_ldtk_entity::<TownBundle>("Town");
    }
}
