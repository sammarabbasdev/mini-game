use bevy::prelude::*;

pub const BACKGROUND_SPRITE: &str = "background.png";
pub const PLAYER_SPRITE: &str = "player_a_01.png";
pub const PLAYER_SIZE: (f32, f32) = (50.0, 50.0);
pub const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
pub const PLAYER_LASER_SIZE: (f32, f32) = (12.0, 30.0);

pub const ENEMY_SPRITE: &str = "enemy_a_01.png";
pub const ENEMY_SIZE: (f32, f32) = (50.0, 50.0);
pub const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
pub const ENEMY_LASER_SIZE: (f32, f32) = (12.0, 30.0);

pub const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
pub const EXPLOSION_LEN: usize = 16;
pub const SPRITE_SCALE: f32 = 0.20;
pub const BASE_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 140.0;
pub const ENEMY_LASER_SPEED: f32 = 0.6;

pub const PLAYER_RESPAWN_DELAY: f64 = 2.0;
pub const ENEMY_MAX: u32 = 6;
pub const FORMATION_MEMBERS_MAX: u32 = 3;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    pub _background: Handle<Image>,
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub explosion_layout: Handle<TextureAtlasLayout>,
    pub explosion_texture: Handle<Image>,
}

#[derive(Resource)]
pub struct EnemyCount(pub u32);

#[derive(Resource)]
pub struct PlayerState {
    pub on: bool,
    pub last_shot: f64,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.0,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }

    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.0;
    }
}
