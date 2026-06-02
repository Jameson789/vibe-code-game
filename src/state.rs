use bevy::prelude::*;

/// Overall game flow.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Aiming,
    BallMoving,
    HoleComplete,
    Penalty,
    GameOver,
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

/// The ball's last at-rest position, used to reset after water/out-of-bounds.
#[derive(Resource)]
pub struct LastRest(pub Vec3);

impl Default for LastRest {
    fn default() -> Self {
        Self(Vec3::new(0.0, 0.3, 6.0))
    }
}

/// Counts down the brief "hazard" message before returning the ball to aiming.
#[derive(Resource)]
pub struct PenaltyTimer(pub Timer);

impl Default for PenaltyTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.2, TimerMode::Once))
    }
}
