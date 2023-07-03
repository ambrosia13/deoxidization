use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
pub const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

pub const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);
pub const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);
pub const COLOR_PLAYER: Color = Color::rgb(0.60, 0.55, 0.60);

pub const FLOOR_THICKNESS: f32 = 10.0;

pub const PLAYER_VELOCITY_X: f32 = 400.0;
pub const PLAYER_VELOCITY_Y: f32 = 850.0;

pub const MAX_JUMP_HEIGHT: f32 = 230.0;
