use bevy::prelude::*;

mod components;
mod physics;
use components::{Ball, Velocity};
use physics::{integrate, is_at_rest};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, ball_physics)
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

    // Camera looking down at the plane from an angle.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // The golf ball: a small white sphere resting on the ground.
    // (Temporary starting velocity so we can watch friction stop it this task.)
    commands.spawn((
        Ball,
        Velocity(Vec3::new(0.0, 0.0, -4.0)),
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.3, 6.0),
    ));
}

fn ball_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity) in &mut query {
        let (new_pos, new_vel) = integrate(transform.translation, velocity.0, 1.2, dt);
        transform.translation = new_pos;
        velocity.0 = new_vel;
        if is_at_rest(velocity.0, 0.05) {
            velocity.0 = Vec3::ZERO;
        }
    }
}
