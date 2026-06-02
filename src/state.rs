use bevy::prelude::*;

/// Overall game flow.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Aiming,
    BallMoving,
    HoleComplete,
}

/// Current aim direction (yaw, radians) and shot power (0.0..=1.0).
#[derive(Resource)]
pub struct AimState {
    pub yaw: f32,
    pub power: f32,
}

impl Default for AimState {
    fn default() -> Self {
        Self { yaw: 0.0, power: 0.5 }
    }
}

/// Stroke count for the current hole.
#[derive(Resource, Default)]
pub struct Strokes(pub u32);
