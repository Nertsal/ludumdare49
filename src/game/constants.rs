use geng::prelude::{vec2, Color, Vec2};

pub const BORDER_SIZE: Vec2<f32> = vec2(100.0, 100.0);

pub const PLAYER_RADIUS: f32 = 2.5;
pub const PLAYER_SPEED: f32 = 50.0;
pub const PLAYER_ACCELERATION: f32 = 300.0;
pub const PLAYER_MASS: f32 = 10.0;
pub const PLAYER_COLOR: Color<f32> = Color::RED; // Color::rgb(1.0, 0.0, 0.0);

pub const REACTOR_RADIUS: f32 = 5.0;
pub const REACTOR_HEALTH: f32 = 10.0;
pub const REACTOR_COLOR: Color<f32> = Color::GREEN;

pub const ASTEROID_COLOR: Color<f32> = Color::GRAY;
pub const ASTEROID_MASS_MIN: f32 = 5.0;
pub const ASTEROID_MASS_MAX: f32 = 20.0;
pub const ASTEROID_RADIUS_MIN: f32 = 2.0;
pub const ASTEROID_RADIUS_MAX: f32 = 5.0;
pub const ASTEROID_SPEED_MIN: f32 = 10.0;
pub const ASTEROID_SPEED_MAX: f32 = 25.0;

pub const SPAWN_DELAY: f32 = 5.0;
