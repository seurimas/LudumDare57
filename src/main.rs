use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkPlugin, LdtkWorldBundle, LevelSelection};
use LudumDare57::{game::map::TownBundle, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Signs of Corruption: The Deep Ones".to_string(),
                resolution: (948., 533.).into(),
                fit_canvas_to_parent: false,
                prevent_default_event_handling: true,
                canvas: Some("#bevy".to_owned()),
                ..Default::default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(LdtkPlugin)
        .register_ldtk_entity::<TownBundle>("Town")
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, spawn_debug_world)
        .run();
}

fn spawn_debug_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("worlds.ldtk").into(),
        ..Default::default()
    });
}
