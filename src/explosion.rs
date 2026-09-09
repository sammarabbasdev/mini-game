use crate::components::*;
use crate::resources::{GameTextures, EXPLOSION_LEN};
use bevy::prelude::*;

pub fn spawn_explosion_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (spawn_entity, explosion_to_spawn) in query.iter() {
        commands.spawn((
            Sprite {
                image: game_textures.explosion_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_textures.explosion_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_translation(explosion_to_spawn.0),
            Explosion,
            ExplosionTimer::default(),
        ));
        commands.entity(spawn_entity).despawn();
    }
}

pub fn animate_explosion_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut Sprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index += 1;
                if atlas.index >= EXPLOSION_LEN {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
