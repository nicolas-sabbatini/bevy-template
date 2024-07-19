#![allow(dead_code)]
use bevy::{
    color::Srgba,
    prelude::{Color, IVec2, KeyCode},
};

// Screen config
pub const GAME_WIDTH: f32 = 800.0;
pub const GAME_HEIGHT: f32 = 600.0;
pub const WINDOW_TITLE: &str = "{{project-name}}";

// Windows camera
pub const WINDOW_CAMERA_CLEAR_COLOR: Color = Color::Srgba(Srgba {
    red: 0.3,
    green: 0.3,
    blue: 0.3,
    alpha: 1.0,
});

// Game camera
pub const GAME_CAMERA_CLEAR_COLOR: Color = Color::Srgba(Srgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 1.0,
});

// Directions
const UP: IVec2 = IVec2::Y;
const DOWN: IVec2 = IVec2::NEG_Y;
const LEFT: IVec2 = IVec2::NEG_X;
const RIGHT: IVec2 = IVec2::X;

// Key maping directions
pub const DIR_KEY_MAPPING: [(KeyCode, IVec2); 4] = [
    (KeyCode::KeyW, UP),
    (KeyCode::KeyS, DOWN),
    (KeyCode::KeyA, LEFT),
    (KeyCode::KeyD, RIGHT),
];
