use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Town {
    pub deep: bool,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct TownBundle {
    pub town: Town,
    #[sprite]
    sprite: Sprite,
}
