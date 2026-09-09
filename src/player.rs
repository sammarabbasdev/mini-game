use crate::components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use crate::resources::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_systems(
                Update,
                player_spawn_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
            )
            .add_systems(Update, (player_control_system, player_fire_system));
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let now = time.elapsed_secs_f64();
    if !player_state.on
        && (player_state.last_shot == -1.0 || now > player_state.last_shot + PLAYER_RESPAWN_DELAY)
    {
        let bottom_y = -win_size.h / 2.0;
        commands.spawn((
            Sprite::from_image(game_textures.player.clone()),
            Transform {
                translation: Vec3::new(
                    0.0,
                    bottom_y + 35.0,
                    10.0,
                ),
                scale: Vec3::splat(SPRITE_SCALE),
                ..default()
            },
            Player,
            SpriteSize::from(PLAYER_SIZE),
            Movable {
                auto_despawn: false,
            },
            Velocity { x: 0.0, y: 0.0 },
        ));
        player_state.spawned();
    }
}

fn player_fire_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_textures: Res<GameTextures>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for offset_x in [-12.0, 0.0, 12.0] {
                commands.spawn((
                    Sprite::from_image(game_textures.player_laser.clone()),
                    Transform {
                        translation: Vec3::new(
                            player_transform.translation.x + offset_x,
                            player_transform.translation.y + 15.0,
                            1.0,
                        ),
                        scale: Vec3::splat(SPRITE_SCALE * 0.5),
                        ..default()
                    },
                    Laser,
                    FromPlayer,
                    SpriteSize::from(PLAYER_LASER_SIZE),
                    Movable { auto_despawn: true },
                    Velocity { x: 0.0, y: 1.2 },
                ));
            }
        }
    }
}

fn player_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut velocity_query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = velocity_query.single_mut() {
        velocity.x = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            1.0
        } else {
            0.0
        };
    }
}
