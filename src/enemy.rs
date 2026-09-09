use crate::components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity};
use crate::resources::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::RngExt;
use std::{f32::consts::PI, time::Duration};

#[derive(Clone, Component)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub pivot: (f32, f32),
    pub speed: f32,
    pub angle: f32,
}

#[derive(Default, Resource)]
pub struct FormationMaker {
    current_template: Option<Formation>,
    current_members: u32,
}

impl FormationMaker {
    pub fn make(&mut self, win_size: &WinSize) -> Formation {
        if let (Some(template), false) = (
            &self.current_template,
            self.current_members >= FORMATION_MEMBERS_MAX,
        ) {
            self.current_members += 1;
            return template.clone();
        }

        let mut rng = rand::rng();
        let (width, height) = (win_size.w, win_size.h);
        let start_x = if rng.random_bool(0.5) {
            width / 2.0 + 100.0
        } else {
            -width / 2.0 - 100.0
        };
        let start_y = rng.random_range((-height / 2.0 - 100.0)..(height / 2.0 + 100.0));
        let pivot = (
            rng.random_range((-width / 4.0)..(width / 4.0)),
            rng.random_range(0.0..(height / 3.0 - 50.0)),
        );
        let formation = Formation {
            start: (start_x, start_y),
            radius: (rng.random_range(80.0..150.0), 100.0),
            pivot,
            speed: ENEMY_SPEED,
            angle: (start_y - pivot.1).atan2(start_x - pivot.0),
        };

        self.current_template = Some(formation.clone());
        self.current_members = 1;
        formation
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FormationMaker::default())
            .add_systems(
                Update,
                enemy_spawn_system.run_if(on_timer(Duration::from_secs(1))),
            )
            .add_systems(
                Update,
                enemy_fire_system.run_if(on_timer(Duration::from_secs_f32(0.6))),
            )
            .add_systems(Update, enemy_movement_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    mut formation_maker: ResMut<FormationMaker>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;
        commands.spawn((
            Sprite::from_image(game_textures.enemy.clone()),
            Transform {
                translation: Vec3::new(x, y, 10.0),
                scale: Vec3::splat(SPRITE_SCALE),
                ..default()
            },
            Enemy,
            formation,
            SpriteSize::from(ENEMY_SIZE),
        ));
        enemy_count.0 += 1;
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for transform in enemy_query.iter() {
        let (x, y) = (transform.translation.x, transform.translation.y);
        commands.spawn((
            Sprite::from_image(game_textures.enemy_laser.clone()),
            Transform {
                translation: Vec3::new(x, y - 15.0, 1.0),
                rotation: Quat::from_rotation_x(PI),
                scale: Vec3::splat(SPRITE_SCALE * 0.5),
            },
            Laser,
            FromEnemy,
            SpriteSize::from(ENEMY_LASER_SIZE),
            Movable { auto_despawn: true },
            Velocity {
                x: 0.0,
                y: -ENEMY_LASER_SPEED,
            },
        ));
    }
}

fn enemy_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>,
) {
    let delta_time = time.delta_secs();

    for (mut transform, mut formation) in &mut query {
        let current_x = transform.translation.x;
        let current_y = transform.translation.y;
        let step_distance = delta_time * formation.speed;

        let orbit_direction: f32 = if formation.start.0 < 0.0 { 1.0 } else { -1.0 };
        let (pivot_x, pivot_y) = formation.pivot;
        let (radius_x, radius_y) = formation.radius;

        let next_angle = formation.angle
            + orbit_direction * formation.speed * delta_time / (radius_x.min(radius_y) * PI / 2.0);
        let target_x = pivot_x + radius_x * next_angle.cos();
        let target_y = pivot_y + radius_y * next_angle.sin();

        let delta_x = current_x - target_x;
        let delta_y = current_y - target_y;
        let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();

        let movement_ratio = if distance == 0.0 {
            0.0
        } else {
            step_distance / distance
        };

        let next_x = if delta_x > 0.0 {
            (current_x - delta_x * movement_ratio).max(target_x)
        } else {
            (current_x - delta_x * movement_ratio).min(target_x)
        };
        let next_y = if delta_y > 0.0 {
            (current_y - delta_y * movement_ratio).max(target_y)
        } else {
            (current_y - delta_y * movement_ratio).min(target_y)
        };

        if distance < step_distance * formation.speed / 20.0 {
            formation.angle = next_angle;
        }

        transform.translation.x = next_x;
        transform.translation.y = next_y;
    }
}
