use crate::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::{rngs::StdRng, seq::IndexedRandom, Rng};

use crate::game::TownState;

#[derive(Resource)]
pub struct TownGenerationRand(pub StdRng);

#[derive(Default, Component)]
pub struct Town {
    pub deep: bool,
    pub name: String,
}

impl From<&EntityInstance> for Town {
    fn from(entity_instance: &EntityInstance) -> Self {
        let deep = if let Ok(deep) = entity_instance.get_bool_field("deep") {
            *deep
        } else {
            false
        };
        let name = if let Ok(name) = entity_instance.get_string_field("name") {
            name.clone()
        } else {
            "Unknown Town".to_string()
        };
        Town { deep, name }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct TownBundle {
    #[from_entity_instance]
    pub town: Town,
    #[sprite_sheet]
    sprite: Sprite,
}

pub fn process_towns(
    mut commands: Commands,
    new_towns: Query<Entity, Added<Town>>,
    town_query: Query<&Town>,
    town_generation_rand: Option<ResMut<TownGenerationRand>>,
) {
    if new_towns.is_empty() {
        return;
    }
    let Some(mut town_generation_rand) = town_generation_rand else {
        return;
    };
    let deep_towns = new_towns
        .iter()
        .filter(|town| town_query.get(*town).unwrap().deep)
        .collect::<Vec<_>>();
    let non_deep_towns = new_towns
        .iter()
        .filter(|town| !town_query.get(*town).unwrap().deep)
        .collect::<Vec<_>>();
    // Remove some deep towns and non-deep towns to keep the number of each in a reasonable range.
    let keep_deeps = town_generation_rand.0.random_range(3..=5);
    let keep_non_deeps = town_generation_rand.0.random_range(3..=5);
    let mut removed = HashSet::new();
    for removed_town in
        deep_towns.choose_multiple(&mut town_generation_rand.0, deep_towns.len() - keep_deeps)
    {
        commands.entity(*removed_town).despawn_recursive();
        removed.insert(*removed_town);
    }
    for removed_town in non_deep_towns.choose_multiple(
        &mut town_generation_rand.0,
        non_deep_towns.len() - keep_non_deeps,
    ) {
        commands.entity(*removed_town).despawn_recursive();
        removed.insert(*removed_town);
    }
    // Add town states to the towns, generating everything needed for the game.
    // Initialize the interactivity.
    for town in new_towns.iter() {
        if removed.contains(&town) {
            continue;
        }
        let deep = town_query.get(town).unwrap().deep;
        if deep {
            commands
                .entity(town)
                .insert(TownState::generate_deep(&mut town_generation_rand.0))
                .observe(handle_pointer_over_town)
                .observe(handle_pointer_out_town)
                .observe(handle_pointer_pressed_town);
        } else {
            commands
                .entity(town)
                .insert(TownState::generate_non_deep(&mut town_generation_rand.0))
                .observe(handle_pointer_over_town)
                .observe(handle_pointer_out_town)
                .observe(handle_pointer_pressed_town);
        }
    }
}
