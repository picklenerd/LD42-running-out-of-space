pub const SCREEN_WIDTH: u32 = 1024;
pub const SCREEN_HEIGHT: u32 = 768;
pub const BACKGROUND_COLOR: u32 = 0xCCCCCC;

pub const PLAYER_COLOR: u32 = 0x000000;
pub const PLAYER_SIZE: u32 = 10;
pub const PLAYER_START_X: f64 = SCREEN_WIDTH as f64 / 2.0;
pub const PLAYER_START_Y: f64 = SCREEN_HEIGHT as f64 / 2.0;
pub const PLAYER_SPEED: f64 = 4.0;
pub const PLAYER_SLOWED_MULTIPLIER: f64 = 0.5;
pub const PLAYER_DECELERATION: f64 = 2.0;

pub const ENEMY_COLOR: u32 = 0xFF0000;
pub const ENEMY_SIZE: u32 = 20;
pub const ENEMY_SPEED: f64 = 5.0;
pub const ENEMY_SLOWED_MULTIPLIER: f64 = 0.5;
pub const ENEMY_MIN_SPAWN_DISTANCE: f64 = 100.0;
pub const ENEMY_HEALTH: i32 = 3;
pub const ENEMY_SCORE: i32 = 1;

pub const MAX_ENEMIES: u8 = 3;
pub const TIME_BETWEEN_SPAWNS: f64 = 2000.0;

pub const PROJECTILE_COLOR: u32 = 0x00AAFF;
pub const PROJECTILE_SIZE: u32 = 10;
pub const PROJECTILE_SPEED: f64 = 10.0;
pub const PROJECTILE_DAMAGE: i32 = 1;

pub const ICE_BLOCK_COLOR: u32 = 0x00AAFF;
pub const ICE_BLOCK_SIZE: u32 = 30;

pub const SPRITE_SIZE_MULTIPLIER: f64 = 4.0;
