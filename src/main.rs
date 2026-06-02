use bevy::prelude::*;

mod camera;
mod components;
mod input;
mod physics;
mod state;
mod ui;
use components::{Ball, Hole, MainCamera, Sand, Slope, Velocity, Wall};
use physics::{integrate, is_at_rest, is_in_hole, reflect, slope_acceleration};
use state::{AimState, GameState, Strokes};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .init_resource::<AimState>()
        .init_resource::<Strokes>()
        .add_systems(Startup, setup)
        .add_systems(Startup, ui::setup_hud)
        .add_systems(Update, ui::update_hud)
        .add_systems(Update, ball_physics.run_if(in_state(GameState::BallMoving)))
        .add_systems(Update, slope_force.run_if(in_state(GameState::BallMoving)))
        .add_systems(Update, slope_indicator)
        .add_systems(Update, wall_collision.run_if(in_state(GameState::BallMoving)))
        .add_systems(Update, hole_check.run_if(in_state(GameState::BallMoving)))
        .add_systems(Update, input::aim_input.run_if(in_state(GameState::Aiming)))
        .add_systems(Update, input::swing.run_if(in_state(GameState::Aiming)))
        .add_systems(Update, camera::chase_camera)
        .add_systems(Update, camera::aim_indicator.run_if(in_state(GameState::Aiming)))
        .add_systems(OnEnter(GameState::HoleComplete), ui::show_win)
        .add_systems(Update, sink_animation.run_if(in_state(GameState::HoleComplete)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground: a 20x20 green plane on the X-Z plane.
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.25, 0.6, 0.25))),
    ));

    // Sun-like directional light.
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Camera (a chase camera repositions it behind the ball every frame).
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // The golf ball: a small white sphere resting on the ground.
    commands.spawn((
        Ball,
        Velocity::default(),
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.3, 6.0),
    ));

    // The hole: a dark flat disc near the far end of the course.
    let hole_pos = Vec3::new(0.0, 0.01, -6.0);
    commands.spawn((
        Hole { radius: 0.6 },
        Mesh3d(meshes.add(Cylinder::new(0.6, 0.02))),
        MeshMaterial3d(materials.add(Color::srgb(0.05, 0.05, 0.05))),
        Transform::from_translation(hole_pos),
    ));

    // Four walls just inside the edges of the 20x20 ground (half-extent 10).
    let wall_specs = [
        (Vec3::new(0.0, 0.5, -10.0), Vec3::new(20.0, 1.0, 0.5), Vec3::Z), // far,  pushes +Z
        (Vec3::new(0.0, 0.5, 10.0), Vec3::new(20.0, 1.0, 0.5), -Vec3::Z), // near, pushes -Z
        (Vec3::new(-10.0, 0.5, 0.0), Vec3::new(0.5, 1.0, 20.0), Vec3::X), // left, pushes +X
        (Vec3::new(10.0, 0.5, 0.0), Vec3::new(0.5, 1.0, 20.0), -Vec3::X), // right, pushes -X
    ];
    let wall_material = materials.add(Color::srgb(0.4, 0.3, 0.2));
    for (pos, size, normal) in wall_specs {
        commands.spawn((
            Wall { normal },
            Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
            MeshMaterial3d(wall_material.clone()),
            Transform::from_translation(pos),
        ));
    }

    // Slope zone: a flat patch flush with the ground (no clipping). Its normal
    // leans toward +X, so the +X side is "downhill" and the ball drifts +X over it.
    // A white arrow (see slope_indicator) shows the downhill direction.
    let slope_normal = Vec3::new(0.4, 1.0, 0.0).normalize();
    commands.spawn((
        Slope {
            normal: slope_normal,
            min: Vec3::new(2.0, 0.0, -8.0),
            max: Vec3::new(8.0, 0.0, 4.0),
        },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(6.0, 12.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.55, 0.35))),
        Transform::from_xyz(5.0, 0.02, -2.0),
    ));

    // A tan sand patch between the ball and the hole; it slows the ball sharply.
    commands.spawn((
        Sand {
            min: Vec3::new(-3.0, 0.0, -3.0),
            max: Vec3::new(3.0, 0.0, -1.0),
        },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(6.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.85, 0.78, 0.5))),
        Transform::from_xyz(0.0, 0.02, -2.0),
    ));
}

fn ball_physics(
    time: Res<Time>,
    sand_q: Query<&Sand>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity) in &mut query {
        let p = transform.translation;
        let in_sand = sand_q
            .iter()
            .any(|s| p.x >= s.min.x && p.x <= s.max.x && p.z >= s.min.z && p.z <= s.max.z);
        let friction = if in_sand { 4.0 } else { 1.2 };
        let (new_pos, new_vel) = integrate(transform.translation, velocity.0, friction, dt);
        transform.translation = new_pos;
        velocity.0 = new_vel;
        if is_at_rest(velocity.0, 0.05) {
            velocity.0 = Vec3::ZERO;
            next_state.set(GameState::Aiming);
        }
    }
}

fn hole_check(
    mut next_state: ResMut<NextState<GameState>>,
    ball_q: Query<(&Transform, &Velocity), With<Ball>>,
    hole_q: Query<(&Transform, &Hole)>,
) {
    let Ok((ball_t, ball_v)) = ball_q.single() else {
        return;
    };
    let Ok((hole_t, hole)) = hole_q.single() else {
        return;
    };
    if is_in_hole(
        ball_t.translation,
        hole_t.translation,
        hole.radius,
        ball_v.0.length(),
        4.0,
    ) {
        next_state.set(GameState::HoleComplete);
    }
}

/// Draws a white downhill arrow over each slope zone so the slope is legible.
fn slope_indicator(mut gizmos: Gizmos, slope_q: Query<&Slope>) {
    for slope in &slope_q {
        let center = (slope.min + slope.max) * 0.5;
        let accel = slope_acceleration(slope.normal, 9.8);
        let downhill = Vec3::new(accel.x, 0.0, accel.z).normalize_or_zero();
        let start = Vec3::new(center.x, 0.15, center.z);
        gizmos.arrow(start, start + downhill * 2.5, Color::srgb(1.0, 1.0, 1.0));
    }
}

fn slope_force(
    time: Res<Time>,
    slope_q: Query<&Slope>,
    mut ball_q: Query<(&Transform, &mut Velocity), With<Ball>>,
) {
    let dt = time.delta_secs();
    let Ok((t, mut v)) = ball_q.single_mut() else {
        return;
    };
    for slope in &slope_q {
        let p = t.translation;
        let inside = p.x >= slope.min.x
            && p.x <= slope.max.x
            && p.z >= slope.min.z
            && p.z <= slope.max.z;
        if inside {
            // Apply only the horizontal drift; the ball stays pinned to the ground.
            let accel = slope_acceleration(slope.normal, 9.8);
            v.0.x += accel.x * dt;
            v.0.z += accel.z * dt;
        }
    }
}

fn wall_collision(
    mut ball_q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    wall_q: Query<&Wall>,
) {
    let half = 9.5_f32; // ball is reflected just inside the walls
    let Ok((mut t, mut v)) = ball_q.single_mut() else {
        return;
    };
    // Reflect off whichever bound was crossed, using the matching wall normal.
    for wall in &wall_q {
        let n = wall.normal;
        if n == Vec3::Z && t.translation.z < -half {
            t.translation.z = -half;
            v.0 = reflect(v.0, n, 0.7);
        } else if n == -Vec3::Z && t.translation.z > half {
            t.translation.z = half;
            v.0 = reflect(v.0, n, 0.7);
        } else if n == Vec3::X && t.translation.x < -half {
            t.translation.x = -half;
            v.0 = reflect(v.0, n, 0.7);
        } else if n == -Vec3::X && t.translation.x > half {
            t.translation.x = half;
            v.0 = reflect(v.0, n, 0.7);
        }
    }
}

/// After sinking, ease the ball toward the hole center and drop it below the
/// surface so it visually falls into the hole.
fn sink_animation(
    time: Res<Time>,
    hole_q: Query<&Transform, (With<Hole>, Without<Ball>)>,
    mut ball_q: Query<&mut Transform, With<Ball>>,
) {
    let Ok(hole) = hole_q.single() else {
        return;
    };
    let Ok(mut ball) = ball_q.single_mut() else {
        return;
    };
    // Target: centered on the hole, sunk below the ground plane.
    let target = Vec3::new(hole.translation.x, -0.4, hole.translation.z);
    let t = (time.delta_secs() * 6.0).min(1.0);
    ball.translation = ball.translation.lerp(target, t);
}
