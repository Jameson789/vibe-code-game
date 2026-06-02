use bevy::math::Vec3;

/// Advance position by velocity and apply rolling friction for one step.
/// Returns the new (position, velocity).
pub fn integrate(position: Vec3, velocity: Vec3, friction: f32, dt: f32) -> (Vec3, Vec3) {
    let new_position = position + velocity * dt;
    let decay = (1.0 - friction * dt).max(0.0);
    let new_velocity = velocity * decay;
    (new_position, new_velocity)
}

/// True when the ball is moving slowly enough to be considered stopped.
pub fn is_at_rest(velocity: Vec3, threshold: f32) -> bool {
    velocity.length() < threshold
}

/// Convert aim yaw + power into a launch velocity. yaw=0 points toward -Z.
pub fn launch_velocity(yaw: f32, power: f32, max_speed: f32) -> Vec3 {
    let speed = power.clamp(0.0, 1.0) * max_speed;
    Vec3::new(yaw.sin() * speed, 0.0, -yaw.cos() * speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrate_advances_position_by_velocity_with_no_friction() {
        let (pos, vel) = integrate(Vec3::ZERO, Vec3::new(2.0, 0.0, 0.0), 0.0, 0.5);
        assert!((pos.x - 1.0).abs() < 1e-6);
        assert!((vel.x - 2.0).abs() < 1e-6);
    }

    #[test]
    fn integrate_reduces_speed_with_friction() {
        let (_, vel) = integrate(Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), 1.0, 0.25);
        assert!(vel.x < 4.0 && vel.x > 0.0);
    }

    #[test]
    fn is_at_rest_true_below_threshold() {
        assert!(is_at_rest(Vec3::new(0.01, 0.0, 0.0), 0.05));
        assert!(!is_at_rest(Vec3::new(1.0, 0.0, 0.0), 0.05));
    }

    #[test]
    fn launch_zero_power_is_zero_velocity() {
        let v = launch_velocity(0.0, 0.0, 10.0);
        assert!(v.length() < 1e-6);
    }

    #[test]
    fn launch_yaw_zero_points_negative_z() {
        let v = launch_velocity(0.0, 1.0, 10.0);
        assert!((v.z + 10.0).abs() < 1e-5);
        assert!(v.x.abs() < 1e-5);
    }
}
