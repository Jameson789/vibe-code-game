use bevy::prelude::*;
use crate::components::{Ball, Velocity};
use crate::physics::launch_velocity;
use crate::state::{AimState, GameState};

/// While aiming, ←/→ rotate aim and ↑/↓ change power. Logs the values so we can
/// verify before any visible aim indicator exists.
pub fn aim_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut aim: ResMut<AimState>,
) {
    let dt = time.delta_secs();
    let turn_speed = 1.5; // radians/sec
    let power_speed = 0.6; // per sec

    if keys.pressed(KeyCode::ArrowLeft) {
        aim.yaw -= turn_speed * dt;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        aim.yaw += turn_speed * dt;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        aim.power = (aim.power + power_speed * dt).clamp(0.0, 1.0);
    }
    if keys.pressed(KeyCode::ArrowDown) {
        aim.power = (aim.power - power_speed * dt).clamp(0.0, 1.0);
    }

    if keys.any_just_pressed([
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
    ]) {
        info!("aim yaw = {:.2} rad, power = {:.0}%", aim.yaw, aim.power * 100.0);
    }
}

/// Spacebar swing: launch the ball from the current aim/power, then enter BallMoving.
pub fn swing(
    keys: Res<ButtonInput<KeyCode>>,
    aim: Res<AimState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Velocity, With<Ball>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let max_speed = 36.0;
        for mut velocity in &mut query {
            velocity.0 = launch_velocity(aim.yaw, aim.power, max_speed);
        }
        next_state.set(GameState::BallMoving);
    }
}
