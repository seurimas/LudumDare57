use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Town {
    pub deep: bool,
}

impl From<&EntityInstance> for Town {
    fn from(entity_instance: &EntityInstance) -> Self {
        let deep = if let Ok(deep) = entity_instance.get_bool_field("deep") {
            *deep
        } else {
            false
        };
        Town { deep }
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
) {
    if new_towns.is_empty() {
        return;
    }
    println!("Processing towns: {:?}", new_towns.iter().count());
    let deep_towns = new_towns
        .iter()
        .filter(|town| town_query.get(*town).unwrap().deep)
        .collect::<Vec<_>>();
    let non_deep_towns = new_towns
        .iter()
        .filter(|town| !town_query.get(*town).unwrap().deep)
        .collect::<Vec<_>>();
    let mut keep_deeps = 3;
    let mut keep_nondeeps = 5;
    println!(
        "Pruning {} deep towns and {} non-deep towns",
        deep_towns.len() - keep_deeps,
        non_deep_towns.len() - keep_nondeeps
    );
    for town in deep_towns {
        if keep_deeps > 0 {
            keep_deeps -= 1;
        } else {
            commands.entity(town).despawn_recursive();
        }
    }
    for town in non_deep_towns {
        if keep_nondeeps > 0 {
            keep_nondeeps -= 1;
        } else {
            commands.entity(town).despawn_recursive();
        }
    }
}
