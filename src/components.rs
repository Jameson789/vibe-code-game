use bevy::prelude::*;

/// The player's golf ball.
#[derive(Component)]
pub struct Ball;

/// Linear velocity in world units per second.
#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

/// Marks the main game camera.
#[derive(Component)]
pub struct MainCamera;

/// The target hole. `radius` is the capture radius on the ground.
#[derive(Component)]
pub struct Hole {
    pub radius: f32,
}

/// An axis-aligned wall. `normal` is the unit direction the wall pushes the ball
/// back toward (pointing into the play area).
#[derive(Component)]
pub struct Wall {
    pub normal: Vec3,
}
