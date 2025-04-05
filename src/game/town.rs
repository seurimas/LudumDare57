use bevy::utils::HashSet;
use rand::{rngs::StdRng, Rng};

use crate::prelude::*;

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct TownState {
    pub is_deep: bool,
    pub deep_deck: Vec<DeepCard>,
    pub civilians: Vec<Civilian>,
    pub agents: Vec<Agent>,
}

impl TownState {
    pub fn generate_deep(rng: &mut StdRng) -> Self {
        let mut deep_deck = Vec::new();
        for _ in 0..rng.random_range(3..=5) {
            deep_deck.push(DeepCard::GiftPowerSmall);
        }
        for _ in 0..rng.random_range(3..=5) {
            deep_deck.push(DeepCard::GiftPowerMedium);
        }
        deep_deck.push(DeepCard::DrawIre);
        let mut civilians = Vec::new();
        for _ in 0..rng.random_range(20..=30) {
            civilians.push(Civilian {
                name: "Civilian".to_string(),
                influence: rng.random_range(0.5..=1.0),
                sign_holder: rng.random_bool(0.25),
                affinity: None,
                identified_by: HashSet::new(),
            });
        }
        // Agents spawned later.
        let agents = Vec::new();
        TownState {
            is_deep: true,
            deep_deck,
            civilians,
            agents,
        }
    }

    pub fn generate_non_deep(rng: &mut StdRng) -> Self {
        let deep_deck = vec![];
        let mut civilians = Vec::new();
        for _ in 0..rng.random_range(20..=30) {
            civilians.push(Civilian {
                name: "Civilian".to_string(),
                influence: rng.random_range(0.5..=1.0),
                sign_holder: rng.random_bool(0.25),
                affinity: None,
                identified_by: HashSet::new(),
            });
        }
        // Agents spawned later.
        let agents = Vec::new();
        TownState {
            is_deep: false,
            deep_deck,
            civilians,
            agents,
        }
    }
}

#[derive(Clone, Debug, Reflect)]
pub enum DeepCard {
    DrawIre,         // Drawer has failed, and the deep will kill them.
    GiftPowerSmall,  // The deep gives the drawer a small power.
    GiftPowerMedium, // The deep gives the drawer a medium power.
    GiftPowerLarge,  // The deep gives the drawer a large power.
}

#[derive(Clone, Debug, Reflect)]
pub struct Civilian {
    pub name: String,
    pub influence: f32,
    pub sign_holder: bool,
    pub affinity: Option<PlayerId>,
    pub identified_by: HashSet<PlayerId>,
}

pub fn handle_pointer_over_town(
    ev: Trigger<Pointer<Over>>,
    mut town: Query<(Entity, &TownState, &mut Sprite)>,
) {
    let Ok((_, town, mut sprite)) = town.get_mut(ev.entity()) else {
        return;
    };
    sprite.color = Color::srgb(0.5, 0.5, 0.5);
}

pub fn handle_pointer_out_town(
    ev: Trigger<Pointer<Out>>,
    mut town: Query<(Entity, &TownState, &mut Sprite)>,
) {
    let Ok((_, town, mut sprite)) = town.get_mut(ev.entity()) else {
        return;
    };
    sprite.color = Color::WHITE;
}

#[derive(Event)]
pub struct OpenTownEvent {
    pub entity: Entity,
}

pub fn handle_pointer_pressed_town(
    ev: Trigger<Pointer<Down>>,
    mut open_town_events: EventWriter<OpenTownEvent>,
) {
    open_town_events.send(OpenTownEvent {
        entity: ev.entity(),
    });
}
