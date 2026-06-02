use bevy::prelude::*;
use crate::state::AimState;

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
