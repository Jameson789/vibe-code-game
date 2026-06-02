use bevy::prelude::*;

/// The player's golf ball.
#[derive(Component)]
pub struct Ball;

/// Linear velocity in world units per second.
#[derive(Component, Default)]
pub struct Velocity(pub Vec3);
