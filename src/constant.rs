use bevy::math::Vec2;

// NOISE SETTINGS
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const HEIGHT_OFFSET: i16 = 60;
pub const HEIGHT_INTENSITY: f32 = 0.5;
pub const NOISE_SCALE: Vec2 = Vec2::ONE;
pub const NOISE_OFFSET: Vec2 = Vec2::ZERO;

// PLAYER SETTINGS
pub const RENDER_DISTANCE: usize = 1;
