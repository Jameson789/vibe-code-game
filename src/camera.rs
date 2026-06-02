use bevy::prelude::*;
use crate::components::{Ball, MainCamera};
use crate::state::AimState;

/// Place the camera behind the ball along the aim direction, looking at the ball.
pub fn chase_camera(
    aim: Res<AimState>,
    ball_q: Query<&Transform, With<Ball>>,
    mut cam_q: Query<&mut Transform, (With<MainCamera>, Without<Ball>)>,
) {
    let Ok(ball) = ball_q.single() else { return };
    let Ok(mut cam) = cam_q.single_mut() else { return };

    // Forward aim direction on the ground (yaw=0 -> -Z).
    let forward = Vec3::new(aim.yaw.sin(), 0.0, -aim.yaw.cos());
    let behind = -forward; // camera sits opposite the aim
    let distance = 8.0;
    let height = 4.0;

    let target = ball.translation + behind * distance + Vec3::Y * height;
    cam.translation = target;
    cam.look_at(ball.translation, Vec3::Y);
}
