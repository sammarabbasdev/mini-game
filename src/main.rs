use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use collision::*;
use enemy::EnemyPlugin;
use explosion::*;
use movement::*;
use player::PlayerPlugin;
use resources::*;

mod collision;
mod components;
mod enemy;
mod explosion;
mod movement;
mod player;
mod resources;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fighting game".into(),
                resolution: WindowResolution::new(598, 676),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((PlayerPlugin, EnemyPlugin))
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                movable_system,
                player_hit_enemy_system,
                enemy_hit_player_system,
                spawn_explosion_system,
                animate_explosion_system,
            ),
        )
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let Ok(primary_window) = window_query.single() else {
        return;
    };
    commands.insert_resource(WinSize {
        w: primary_window.width(),
        h: primary_window.height(),
    });

    let explosion_atlas = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 4, None, None);
    let explosion_layout_handle = texture_atlases.add(explosion_atlas);

    let background_handle = asset_server.load(BACKGROUND_SPRITE);

    commands.spawn((
        Sprite {
            image: background_handle.clone(),
            custom_size: Some(Vec2::new(primary_window.width(), primary_window.height())),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.insert_resource(GameTextures {
        _background: background_handle,
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion_layout: explosion_layout_handle,
        explosion_texture: asset_server.load(EXPLOSION_SHEET),
    });
    commands.insert_resource(EnemyCount(0));
}
