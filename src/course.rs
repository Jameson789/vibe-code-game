use bevy::prelude::*;

/// Static description of one hole: where the ball starts and where the hole is.
pub struct HoleLayout {
    pub ball_start: Vec3,
    pub hole_pos: Vec3,
    pub hole_radius: f32,
}

/// The full course. Add or tweak entries to change the game.
pub fn course() -> Vec<HoleLayout> {
    vec![
        HoleLayout {
            ball_start: Vec3::new(0.0, 0.3, 6.0),
            hole_pos: Vec3::new(0.0, 0.01, -6.0),
            hole_radius: 0.6,
        },
        HoleLayout {
            ball_start: Vec3::new(-6.0, 0.3, 6.0),
            hole_pos: Vec3::new(6.0, 0.01, -6.0),
            hole_radius: 0.6,
        },
        HoleLayout {
            ball_start: Vec3::new(6.0, 0.3, 7.0),
            hole_pos: Vec3::new(-5.0, 0.01, -7.0),
            hole_radius: 0.5,
        },
    ]
}

/// Which hole is currently active and the running total.
#[derive(Resource, Default)]
pub struct Course {
    pub index: usize,
    pub total_strokes: u32,
}
