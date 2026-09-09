use crate::components::*;
use crate::resources::{EnemyCount, PlayerState};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn player_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let mut hit_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_transform, laser_size) in laser_query.iter() {
        if hit_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = laser_transform.scale.xy();

        for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
            if hit_entities.contains(&enemy_entity) || hit_entities.contains(&laser_entity) {
                continue;
            }

            let enemy_scale = enemy_transform.scale.xy();

            let collision = Aabb2d::new(
                laser_transform.translation.truncate(),
                (laser_size.0 * laser_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                enemy_transform.translation.truncate(),
                (enemy_size.0 * enemy_scale) / 2.0,
            ));

            if collision {
                commands.entity(enemy_entity).despawn();
                commands.entity(laser_entity).despawn();
                hit_entities.insert(enemy_entity);
                hit_entities.insert(laser_entity);
                enemy_count.0 -= 1;
                commands.spawn(ExplosionToSpawn(enemy_transform.translation));
            }
        }
    }
}

pub fn enemy_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_entity, player_transform, player_size)) = player_query.single() {
        let player_scale = player_transform.scale.xy();

        for (laser_entity, laser_transform, laser_size) in laser_query.iter() {
            let laser_scale = laser_transform.scale.xy();

            let collision = Aabb2d::new(
                laser_transform.translation.truncate(),
                (laser_size.0 * laser_scale) / 2.0,
            )
            .intersects(&Aabb2d::new(
                player_transform.translation.truncate(),
                (player_size.0 * player_scale) / 2.0,
            ));

            if collision {
                commands.entity(player_entity).despawn();
                commands.entity(laser_entity).despawn();
                player_state.shot(time.elapsed_secs_f64());
                commands.spawn(ExplosionToSpawn(player_transform.translation));
                break;
            }
        }
    }
}
