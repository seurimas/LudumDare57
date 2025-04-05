use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkPlugin, LdtkWorldBundle, LevelSelection};
use LudumDare57::{
    game::{map::TownBundle, GamePlugin},
    prelude::*,
};

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
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_debug_world)
        .add_systems(Update, move_camera)
        .run();
}

fn spawn_debug_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("worlds.ldtk").into(),
        ..Default::default()
    });
}

fn move_camera(
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &Camera)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, _camera) = camera.single_mut();
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        direction += Vec3::Y;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        direction -= Vec3::Y;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }
    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * 500. * time.delta_secs();
    }
}
