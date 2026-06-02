# Simple 3D Golf — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a behind-the-ball 3D golf game in Bevy where you aim with ←/→, set power with ↑/↓, and press Space to roll a ball into a hole in as few strokes as possible.

**Architecture:** A Bevy ECS app. Pure, unit-tested math lives in `src/physics.rs` (integration, launch velocity, wall reflection, slope force, out-of-bounds, hole detection). Everything else is thin Bevy systems split by responsibility (`input`, `camera`, `course`, `ui`, `state`). Game flow is driven by a `GameState` state machine (`Aiming → BallMoving → HoleComplete → NextHole`). Custom lightweight physics — no physics engine.

**Tech Stack:** Rust (stable, via rustup.rs), Bevy (latest stable, pinned by `cargo add bevy`), Git.

---

## Conventions (read before starting)

- **Verify every step by running it.** "It compiles" is NOT "it works." After each visual step, run the game and confirm what you see with your own eyes before committing.
- **Commit the moment a step works.** Every task ends with a commit. Use the exact messages given.
- **Bevy API version note:** The Bevy code below targets the **modern Bevy API (0.15+): required components with `Mesh3d` / `MeshMaterial3d` / `Camera3d` / `DirectionalLight` spawned as plain components**, not the old `PbrBundle`/`Camera3dBundle`. Task 1 pins the actual version. If the installed version differs, ask the agent to adapt the *spawning syntax* — the structure of each task stays the same. The pure-math functions in `physics.rs` are version-independent.
- **Pure logic is TDD.** Math functions get a failing `cargo test` first, then the implementation. Visual behavior is verified by run-and-watch (there's no practical headless assertion for "the ball looks like it rolls").
- **Coordinate convention:** Ground is the X-Z plane, +Y is up. Aim `yaw = 0` points toward `-Z`; increasing yaw rotates toward `+X`. The ball starts near `+Z` looking toward `-Z`.

---

## Phase 1 — Bare MVP

### Task 1: Project scaffold + empty Bevy window

**Files:**
- Create: `Cargo.toml` (via `cargo init`)
- Create: `src/main.rs`
- Modify: `.gitignore`

- [ ] **Step 1: Confirm Rust is installed**

Run: `rustc --version && cargo --version`
Expected: both print versions. If `rustc` is missing, install from **https://rustup.rs** (the official installer — do not paste any other install command), then re-run.

- [ ] **Step 2: Initialize the Rust project in the existing repo**

Run from the repo root (`~/sdev378/vibe-code-game`): `cargo init --name simple_3d_golf`
Expected: creates `Cargo.toml` and `src/main.rs`. (The repo and git already exist; `cargo init` will not overwrite them.)

- [ ] **Step 3: Ignore Rust build artifacts**

Add these lines to `.gitignore` (keep the existing markdown-ignore lines):

```gitignore
/target
```

(Do NOT ignore `Cargo.lock` — this is a binary, so the lock file is committed.)

- [ ] **Step 4: Add Bevy and pin the version**

Run: `cargo add bevy`
Then open `Cargo.toml` and confirm a pinned version line exists, e.g. `bevy = "0.16"` (whatever resolved). Add a fast-iteration dev profile to the bottom of `Cargo.toml`:

```toml
# Compile dependencies with optimizations even in dev builds, so the game runs smoothly.
[profile.dev.package."*"]
opt-level = 3
```

- [ ] **Step 5: Write the minimal window app**

Replace `src/main.rs` with:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

- [ ] **Step 6: Run and watch**

Run: `cargo run`
Expected: first build is slow (minutes). A blank window titled with the app name opens. Close it.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml Cargo.lock .gitignore src/main.rs
git commit -m "feat: scaffold Bevy project with empty window"
```

---

### Task 2: Lit ground plane + camera

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Add a setup system that spawns a camera, light, and ground plane**

Replace `src/main.rs` with:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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
}
```

- [ ] **Step 2: Run and watch**

Run: `cargo run`
Expected: a green ground plane viewed from above and behind at an angle, lit from one side. Close the window.

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: add ground plane, light, and angled camera"
```

---

### Task 3: Spawn the ball

**Files:**
- Create: `src/components.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Create the components module with a `Ball` marker**

Create `src/components.rs`:

```rust
use bevy::prelude::*;

/// The player's golf ball.
#[derive(Component)]
pub struct Ball;
```

- [ ] **Step 2: Register the module and spawn a ball sphere**

In `src/main.rs`, add `mod components;` near the top (under the `use` line) and `use components::Ball;`. Then add this to the end of the `setup` function body:

```rust
    // The golf ball: a small white sphere resting on the ground.
    commands.spawn((
        Ball,
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.3, 6.0),
    ));
```

- [ ] **Step 3: Run and watch**

Run: `cargo run`
Expected: a white ball sits on the green plane in the foreground. Close the window.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/components.rs
git commit -m "feat: spawn the golf ball"
```

---

### Task 4: Physics module — integration with friction (TDD)

**Files:**
- Create: `src/physics.rs`
- Modify: `src/main.rs`, `src/components.rs`

- [ ] **Step 1: Write the failing test for `integrate`**

Create `src/physics.rs`:

```rust
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
}
```

- [ ] **Step 2: Register the module and run the tests**

Add `mod physics;` near the top of `src/main.rs`.
Run: `cargo test`
Expected: PASS (3 tests). (These functions are already implemented above; the test proves the math, which is the unit we care about.)

- [ ] **Step 3: Add a `Velocity` component**

Append to `src/components.rs`:

```rust
/// Linear velocity in world units per second.
#[derive(Component, Default)]
pub struct Velocity(pub Vec3);
```

- [ ] **Step 4: Add a velocity to the ball and a physics system that rolls it**

In `src/main.rs`:
- add `use components::Velocity;` and `use physics::{integrate, is_at_rest};`
- give the ball a temporary starting velocity so we can watch it roll: change the ball spawn to include `Velocity(Vec3::new(0.0, 0.0, -4.0)),`
- register the system: `.add_systems(Update, ball_physics)`
- add the system:

```rust
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
```

- [ ] **Step 5: Run and watch**

Run: `cargo run`
Expected: the ball rolls forward (toward -Z) and smoothly slows to a stop. Close the window.

- [ ] **Step 6: Commit**

```bash
git add src/main.rs src/physics.rs src/components.rs
git commit -m "feat: custom velocity+friction physics, ball rolls and stops"
```

---

### Task 5: Game state, aim, and power input

**Files:**
- Create: `src/state.rs`
- Create: `src/input.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Create the state module**

Create `src/state.rs`:

```rust
use bevy::prelude::*;

/// Overall game flow.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Aiming,
    BallMoving,
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
```

- [ ] **Step 2: Create the input module (aim + power only, no swing yet)**

Create `src/input.rs`:

```rust
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
        KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowUp, KeyCode::ArrowDown,
    ]) {
        info!("aim yaw = {:.2} rad, power = {:.0}%", aim.yaw, aim.power * 100.0);
    }
}
```

- [ ] **Step 3: Wire state + input into the app**

In `src/main.rs`:
- add `mod state;` and `mod input;`
- add `use state::{AimState, GameState};`
- in `App::new()` add: `.init_state::<GameState>()` and `.init_resource::<AimState>()`
- register input only while aiming: `.add_systems(Update, input::aim_input.run_if(in_state(GameState::Aiming)))`
- remove the temporary `Velocity(Vec3::new(0.0, 0.0, -4.0))` from the ball spawn — change it back to `Velocity::default(),` so the ball starts at rest.

- [ ] **Step 4: Run and watch**

Run: `cargo run`
Expected: the ball sits still. Pressing arrow keys prints lines like `aim yaw = 0.05 rad, power = 53%` in the terminal. Close the window.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs src/state.rs src/input.rs
git commit -m "feat: game state plus aim/power input"
```

---

### Task 6: Swing — launch the ball (TDD) and complete the MVP loop

**Files:**
- Modify: `src/physics.rs`, `src/input.rs`, `src/main.rs`

- [ ] **Step 1: Write the failing test for `launch_velocity`**

Add to `src/physics.rs` (above the `#[cfg(test)]` block):

```rust
/// Convert aim yaw + power into a launch velocity. yaw=0 points toward -Z.
pub fn launch_velocity(yaw: f32, power: f32, max_speed: f32) -> Vec3 {
    let speed = power.clamp(0.0, 1.0) * max_speed;
    Vec3::new(yaw.sin() * speed, 0.0, -yaw.cos() * speed)
}
```

Add these tests inside the `tests` module:

```rust
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
```

- [ ] **Step 2: Run the tests**

Run: `cargo test`
Expected: PASS (5 tests total).

- [ ] **Step 3: Add a swing system**

Append to `src/input.rs`:

```rust
use crate::components::{Ball, Velocity};
use crate::state::GameState;
use crate::physics::launch_velocity;

/// Spacebar swing: launch the ball from the current aim/power, then enter BallMoving.
pub fn swing(
    keys: Res<ButtonInput<KeyCode>>,
    aim: Res<AimState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Velocity, With<Ball>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let max_speed = 12.0;
        for mut velocity in &mut query {
            velocity.0 = launch_velocity(aim.yaw, aim.power, max_speed);
        }
        next_state.set(GameState::BallMoving);
    }
}
```

- [ ] **Step 4: Add a "ball stopped → back to aiming" transition**

In `src/main.rs`, change `ball_physics` so it reports when the ball stops, and only runs while moving. Replace the `ball_physics` function with:

```rust
fn ball_physics(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity) in &mut query {
        let (new_pos, new_vel) = integrate(transform.translation, velocity.0, 1.2, dt);
        transform.translation = new_pos;
        velocity.0 = new_vel;
        if is_at_rest(velocity.0, 0.05) {
            velocity.0 = Vec3::ZERO;
            next_state.set(GameState::Aiming);
        }
    }
}
```

- [ ] **Step 5: Wire the swing system and gate physics by state**

In `src/main.rs` update the systems registration:
- `.add_systems(Update, input::swing.run_if(in_state(GameState::Aiming)))`
- change physics to `.add_systems(Update, ball_physics.run_if(in_state(GameState::BallMoving)))`

- [ ] **Step 6: Run and watch (full MVP loop)**

Run: `cargo run`
Expected: aim with ←/→, set power with ↑/↓ (watch the % in the terminal), press Space — the ball rolls off in the aimed direction with power-scaled speed, slows, stops, and you can immediately aim and swing again. Close the window.

- [ ] **Step 7: Commit**

```bash
git add src/main.rs src/physics.rs src/input.rs
git commit -m "feat: spacebar swing completes the bare MVP golf loop"
```

**✅ Bare MVP complete.**

---

## Phase 2 — Chase camera + aim indicator

### Task 7: Behind-the-ball chase camera

**Files:**
- Create: `src/camera.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Mark the camera so we can find it**

In `src/main.rs`, change the camera spawn to add a marker component (define it in `components.rs` first):

Append to `src/components.rs`:

```rust
/// Marks the main game camera.
#[derive(Component)]
pub struct MainCamera;
```

Then in `setup`, change the camera spawn to:

```rust
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
```

- [ ] **Step 2: Create the chase-camera system**

Create `src/camera.rs`:

```rust
use bevy::prelude::*;
use crate::components::{Ball, MainCamera};
use crate::state::AimState;

/// Place the camera behind the ball along the aim direction, looking at the ball.
pub fn chase_camera(
    aim: Res<AimState>,
    ball_q: Query<&Transform, With<Ball>>,
    mut cam_q: Query<&mut Transform, (With<MainCamera>, Without<Ball>)>,
) {
    let Ok(ball) = ball_q.get_single() else { return };
    let Ok(mut cam) = cam_q.get_single_mut() else { return };

    // Forward aim direction on the ground (yaw=0 -> -Z).
    let forward = Vec3::new(aim.yaw.sin(), 0.0, -aim.yaw.cos());
    let behind = -forward; // camera sits opposite the aim
    let distance = 8.0;
    let height = 4.0;

    let target = ball.translation + behind * distance + Vec3::Y * height;
    cam.translation = target;
    cam.look_at(ball.translation, Vec3::Y);
}
```

- [ ] **Step 3: Wire it up**

In `src/main.rs`: add `mod camera;`, `use components::MainCamera;`, and register `.add_systems(Update, camera::chase_camera)`.

- [ ] **Step 4: Run and watch**

Run: `cargo run`
Expected: the camera sits behind the ball looking down the shot line. Pressing ←/→ swings the whole view around the ball. After a swing the camera follows the ball's position. Close the window.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs src/camera.rs src/components.rs
git commit -m "feat: behind-the-ball chase camera that follows aim"
```

---

### Task 8: Aim indicator line

**Files:**
- Modify: `src/camera.rs` (add a gizmo system), `src/main.rs`

- [ ] **Step 1: Draw an aim line with gizmos**

Append to `src/camera.rs`:

```rust
/// Draws a line from the ball in the aim direction, length scaled by power.
pub fn aim_indicator(
    mut gizmos: Gizmos,
    aim: Res<AimState>,
    ball_q: Query<&Transform, With<Ball>>,
) {
    let Ok(ball) = ball_q.get_single() else { return };
    let forward = Vec3::new(aim.yaw.sin(), 0.0, -aim.yaw.cos());
    let length = 1.0 + aim.power * 4.0;
    let start = ball.translation;
    let end = start + forward * length;
    gizmos.line(start, end, Color::srgb(1.0, 1.0, 0.0));
}
```

- [ ] **Step 2: Run the indicator only while aiming**

In `src/main.rs`: `.add_systems(Update, camera::aim_indicator.run_if(in_state(GameState::Aiming)))`

- [ ] **Step 3: Run and watch**

Run: `cargo run`
Expected: a yellow line points out from the ball in the aim direction; it rotates with ←/→ and grows/shrinks with ↑/↓. It disappears while the ball is rolling and returns when it stops. Close the window.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/camera.rs
git commit -m "feat: yellow aim indicator scaled by power"
```

---

## Phase 3 — Hole, sink detection, strokes, win screen

### Task 9: Spawn the hole

**Files:**
- Modify: `src/components.rs`, `src/main.rs`

- [ ] **Step 1: Add a `Hole` component**

Append to `src/components.rs`:

```rust
/// The target hole. `radius` is the capture radius on the ground.
#[derive(Component)]
pub struct Hole {
    pub radius: f32,
}
```

- [ ] **Step 2: Spawn a visible hole**

In `src/main.rs`, add `use components::Hole;` and append to `setup`:

```rust
    // The hole: a dark flat disc near the far end of the course.
    let hole_pos = Vec3::new(0.0, 0.01, -6.0);
    commands.spawn((
        Hole { radius: 0.6 },
        Mesh3d(meshes.add(Cylinder::new(0.6, 0.02))),
        MeshMaterial3d(materials.add(Color::srgb(0.05, 0.05, 0.05))),
        Transform::from_translation(hole_pos),
    ));
```

- [ ] **Step 3: Run and watch**

Run: `cargo run`
Expected: a dark circle sits on the green at the far end. Close the window.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/components.rs
git commit -m "feat: spawn the target hole"
```

---

### Task 10: Stroke counter + power/stroke UI

**Files:**
- Create: `src/ui.rs`
- Modify: `src/state.rs`, `src/input.rs`, `src/main.rs`

- [ ] **Step 1: Add a `Strokes` resource**

Append to `src/state.rs`:

```rust
/// Stroke count for the current hole.
#[derive(Resource, Default)]
pub struct Strokes(pub u32);
```

- [ ] **Step 2: Increment strokes on each swing**

In `src/input.rs`, update `swing` to take and bump strokes. Change its signature and body:

```rust
pub fn swing(
    keys: Res<ButtonInput<KeyCode>>,
    aim: Res<AimState>,
    mut strokes: ResMut<crate::state::Strokes>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Velocity, With<Ball>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let max_speed = 12.0;
        for mut velocity in &mut query {
            velocity.0 = launch_velocity(aim.yaw, aim.power, max_speed);
        }
        strokes.0 += 1;
        next_state.set(GameState::BallMoving);
    }
}
```

- [ ] **Step 3: Create the UI module**

Create `src/ui.rs`:

```rust
use bevy::prelude::*;
use crate::state::{AimState, Strokes};

/// Marker for the heads-up text.
#[derive(Component)]
pub struct HudText;

/// Spawn the HUD once at startup.
pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        HudText,
        Text::new("Strokes: 0\nPower: 50%"),
        TextFont { font_size: 22.0, ..default() },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

/// Keep the HUD text in sync with strokes + power.
pub fn update_hud(
    strokes: Res<Strokes>,
    aim: Res<AimState>,
    mut query: Query<&mut Text, With<HudText>>,
) {
    for mut text in &mut query {
        **text = format!("Strokes: {}\nPower: {:.0}%", strokes.0, aim.power * 100.0);
    }
}
```

- [ ] **Step 4: Wire UI into the app**

In `src/main.rs`: add `mod ui;`, `.init_resource::<Strokes>()` (add `use state::Strokes;`), `.add_systems(Startup, ui::setup_hud)`, and `.add_systems(Update, ui::update_hud)`.

- [ ] **Step 5: Run and watch**

Run: `cargo run`
Expected: top-left text shows live `Power: NN%` as you hold ↑/↓, and `Strokes: N` increments by 1 each time you press Space. Close the window.

- [ ] **Step 6: Commit**

```bash
git add src/main.rs src/state.rs src/input.rs src/ui.rs
git commit -m "feat: HUD with live power and stroke count"
```

---

### Task 11: Sink detection + win screen (TDD)

**Files:**
- Modify: `src/physics.rs`, `src/state.rs`, `src/main.rs`, `src/ui.rs`

- [ ] **Step 1: Write the failing test for `is_in_hole`**

Add to `src/physics.rs` (above the test module):

```rust
/// True when the ball is over the hole (horizontal distance < radius) and slow
/// enough to drop in rather than skip over.
pub fn is_in_hole(ball: Vec3, hole: Vec3, hole_radius: f32, speed: f32, capture_speed: f32) -> bool {
    let dx = ball.x - hole.x;
    let dz = ball.z - hole.z;
    let horizontal = (dx * dx + dz * dz).sqrt();
    horizontal < hole_radius && speed < capture_speed
}
```

Add tests inside the `tests` module:

```rust
    #[test]
    fn in_hole_when_close_and_slow() {
        let b = Vec3::new(0.1, 0.3, -6.0);
        let h = Vec3::new(0.0, 0.0, -6.0);
        assert!(is_in_hole(b, h, 0.6, 1.0, 3.0));
    }

    #[test]
    fn not_in_hole_when_too_fast() {
        let b = Vec3::new(0.1, 0.3, -6.0);
        let h = Vec3::new(0.0, 0.0, -6.0);
        assert!(!is_in_hole(b, h, 0.6, 9.0, 3.0));
    }

    #[test]
    fn not_in_hole_when_far() {
        let b = Vec3::new(3.0, 0.3, -6.0);
        let h = Vec3::new(0.0, 0.0, -6.0);
        assert!(!is_in_hole(b, h, 0.6, 0.5, 3.0));
    }
```

- [ ] **Step 2: Run the tests**

Run: `cargo test`
Expected: PASS (8 tests total).

- [ ] **Step 3: Add a `HoleComplete` state**

In `src/state.rs`, add a variant to `GameState`:

```rust
    HoleComplete,
```

(Place it after `BallMoving`.)

- [ ] **Step 4: Add the sink-check system**

In `src/main.rs`, add `use physics::is_in_hole;` and `use components::Hole;` (if not already imported), then add this system and register it to run while the ball is moving:

```rust
fn hole_check(
    mut next_state: ResMut<NextState<GameState>>,
    ball_q: Query<(&Transform, &Velocity), With<Ball>>,
    hole_q: Query<(&Transform, &Hole)>,
) {
    let Ok((ball_t, ball_v)) = ball_q.get_single() else { return };
    let Ok((hole_t, hole)) = hole_q.get_single() else { return };
    if is_in_hole(ball_t.translation, hole_t.translation, hole.radius, ball_v.0.length(), 4.0) {
        next_state.set(GameState::HoleComplete);
    }
}
```

Register: `.add_systems(Update, hole_check.run_if(in_state(GameState::BallMoving)))`

- [ ] **Step 5: Add a win banner shown on entering HoleComplete**

Append to `src/ui.rs`:

```rust
use crate::state::GameState;

#[derive(Component)]
pub struct WinBanner;

/// Show a centered win message when the hole is completed.
pub fn show_win(mut commands: Commands, strokes: Res<Strokes>) {
    commands.spawn((
        WinBanner,
        Text::new(format!("Hole complete in {} strokes!", strokes.0)),
        TextFont { font_size: 40.0, ..default() },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(45.0),
            left: Val::Percent(30.0),
            ..default()
        },
    ));
}
```

- [ ] **Step 6: Run the banner on state entry**

In `src/main.rs`: `.add_systems(OnEnter(GameState::HoleComplete), ui::show_win)`

- [ ] **Step 7: Run and watch**

Run: `cargo run`
Expected: aim at the hole, swing with moderate power, and when the ball settles over the dark disc a large "Hole complete in N strokes!" message appears. (Too much power and it rolls past — that's correct.) Close the window.

- [ ] **Step 8: Commit**

```bash
git add src/main.rs src/physics.rs src/state.rs src/ui.rs
git commit -m "feat: sink detection and win screen — single hole playable"
```

**✅ Playable single-hole golf game complete.**

---

## Phase 4 — Walls & slopes

### Task 12: Spawn border walls

**Files:**
- Modify: `src/components.rs`, `src/main.rs`

- [ ] **Step 1: Add a `Wall` component carrying its axis-aligned bounds**

Append to `src/components.rs`:

```rust
/// An axis-aligned wall. `normal` is the unit direction the wall pushes the ball
/// back toward (pointing into the play area).
#[derive(Component)]
pub struct Wall {
    pub normal: Vec3,
}
```

- [ ] **Step 2: Spawn four border walls around the 20x20 ground**

In `src/main.rs`, add `use components::Wall;` and append a helper call in `setup`. Add this code to `setup`:

```rust
    // Four walls just inside the edges of the 20x20 ground (half-extent 10).
    let wall_specs = [
        (Vec3::new(0.0, 0.5, -10.0), Vec3::new(20.0, 1.0, 0.5), Vec3::Z),   // far,  pushes +Z
        (Vec3::new(0.0, 0.5, 10.0),  Vec3::new(20.0, 1.0, 0.5), -Vec3::Z),  // near, pushes -Z
        (Vec3::new(-10.0, 0.5, 0.0), Vec3::new(0.5, 1.0, 20.0), Vec3::X),   // left, pushes +X
        (Vec3::new(10.0, 0.5, 0.0),  Vec3::new(0.5, 1.0, 20.0), -Vec3::X),  // right,pushes -X
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
```

- [ ] **Step 3: Run and watch**

Run: `cargo run`
Expected: four low brown walls frame the green play area. Close the window.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/components.rs
git commit -m "feat: border walls around the course"
```

---

### Task 13: Wall bounce (TDD)

**Files:**
- Modify: `src/physics.rs`, `src/main.rs`

- [ ] **Step 1: Write the failing test for `reflect`**

Add to `src/physics.rs` (above the test module):

```rust
/// Reflect a velocity off a surface with the given (unit) normal, scaled by
/// restitution (1.0 = perfectly bouncy, 0.0 = dead stop into the wall).
pub fn reflect(velocity: Vec3, normal: Vec3, restitution: f32) -> Vec3 {
    let n = normal.normalize();
    let reflected = velocity - 2.0 * velocity.dot(n) * n;
    reflected * restitution
}
```

Add tests inside the `tests` module:

```rust
    #[test]
    fn reflect_reverses_along_normal() {
        let v = reflect(Vec3::new(0.0, 0.0, -5.0), Vec3::Z, 1.0);
        assert!((v.z - 5.0).abs() < 1e-5);
    }

    #[test]
    fn reflect_applies_restitution() {
        let v = reflect(Vec3::new(0.0, 0.0, -5.0), Vec3::Z, 0.5);
        assert!((v.z - 2.5).abs() < 1e-5);
    }
```

- [ ] **Step 2: Run the tests**

Run: `cargo test`
Expected: PASS (10 tests total).

- [ ] **Step 3: Bounce the ball off walls in the physics step**

In `src/main.rs`, add `use physics::reflect;` and `use components::Wall;`. Add a wall-collision system that runs while the ball moves:

```rust
fn wall_collision(
    mut ball_q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    wall_q: Query<&Wall>,
) {
    let half = 9.5_f32; // ball is reflected just inside the walls
    let Ok((mut t, mut v)) = ball_q.get_single_mut() else { return };
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
```

Register: `.add_systems(Update, wall_collision.run_if(in_state(GameState::BallMoving)))`

- [ ] **Step 4: Run and watch**

Run: `cargo run`
Expected: hit the ball hard at a wall and it bounces back, losing some speed each bounce. Close the window.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs src/physics.rs
git commit -m "feat: ball bounces off walls with restitution"
```

---

### Task 14: Slope force (TDD)

**Files:**
- Modify: `src/physics.rs`, `src/components.rs`, `src/main.rs`

- [ ] **Step 1: Write the failing test for `slope_acceleration`**

Add to `src/physics.rs` (above the test module):

```rust
/// Downhill acceleration on a surface with the given (unit) normal under gravity.
/// Returns the gravity component tangent to the slope (zero on flat ground).
pub fn slope_acceleration(normal: Vec3, gravity: f32) -> Vec3 {
    let n = normal.normalize();
    let g = Vec3::new(0.0, -gravity, 0.0);
    g - g.dot(n) * n
}
```

Add tests inside the `tests` module:

```rust
    #[test]
    fn flat_ground_has_no_slope_force() {
        let a = slope_acceleration(Vec3::Y, 9.8);
        assert!(a.length() < 1e-5);
    }

    #[test]
    fn tilted_ground_pushes_downhill() {
        // Normal tilted toward +X means downhill is -X.
        let normal = Vec3::new(0.3, 1.0, 0.0).normalize();
        let a = slope_acceleration(normal, 9.8);
        assert!(a.x < 0.0);
        assert!(a.length() > 0.1);
    }
```

- [ ] **Step 2: Run the tests**

Run: `cargo test`
Expected: PASS (12 tests total).

- [ ] **Step 3: Add a `Slope` region component**

Append to `src/components.rs`:

```rust
/// A rectangular sloped region of the course. `normal` is the surface normal;
/// `min`/`max` are X-Z bounds (Y ignored).
#[derive(Component)]
pub struct Slope {
    pub normal: Vec3,
    pub min: Vec3,
    pub max: Vec3,
}
```

- [ ] **Step 4: Spawn one visible slope region and apply its force**

In `src/main.rs`, add `use components::Slope;` and `use physics::slope_acceleration;`. In `setup`, spawn a tilted ramp panel and its region marker:

```rust
    // A sloped panel on the right half of the course, tilted so the ball drifts left.
    let slope_normal = Vec3::new(0.4, 1.0, 0.0).normalize();
    commands.spawn((
        Slope { normal: slope_normal, min: Vec3::new(2.0, 0.0, -8.0), max: Vec3::new(8.0, 0.0, 4.0) },
        Mesh3d(meshes.add(Cuboid::new(6.0, 0.1, 12.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.5, 0.2))),
        Transform::from_xyz(5.0, 0.05, -2.0)
            .with_rotation(Quat::from_rotation_z(-0.38)),
    ));
```

Add a system that adds downhill acceleration while the ball is over a slope region:

```rust
fn slope_force(
    time: Res<Time>,
    slope_q: Query<&Slope>,
    mut ball_q: Query<(&Transform, &mut Velocity), With<Ball>>,
) {
    let dt = time.delta_secs();
    let Ok((t, mut v)) = ball_q.get_single_mut() else { return };
    for slope in &slope_q {
        let p = t.translation;
        let inside = p.x >= slope.min.x && p.x <= slope.max.x
            && p.z >= slope.min.z && p.z <= slope.max.z;
        if inside {
            let accel = slope_acceleration(slope.normal, 9.8);
            v.0 += accel * dt;
        }
    }
}
```

Register: `.add_systems(Update, slope_force.run_if(in_state(GameState::BallMoving)))`

- [ ] **Step 5: Run and watch**

Run: `cargo run`
Expected: a tilted green panel sits on the right; rolling the ball across it makes the ball curve downhill (toward -X) instead of going straight. Close the window.

- [ ] **Step 6: Commit**

```bash
git add src/main.rs src/physics.rs src/components.rs
git commit -m "feat: sloped region applies downhill force to the ball"
```

---

## Phase 5 — Obstacles & hazards

### Task 15: Sand traps (extra friction)

**Files:**
- Modify: `src/components.rs`, `src/main.rs`

- [ ] **Step 1: Add a `Sand` region component**

Append to `src/components.rs`:

```rust
/// A rectangular sand region (X-Z bounds) that slows the ball.
#[derive(Component)]
pub struct Sand {
    pub min: Vec3,
    pub max: Vec3,
}
```

- [ ] **Step 2: Spawn a sand patch**

In `src/main.rs`, add `use components::Sand;`. In `setup`:

```rust
    // A tan sand patch between the ball and the hole.
    commands.spawn((
        Sand { min: Vec3::new(-3.0, 0.0, -3.0), max: Vec3::new(3.0, 0.0, -1.0) },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(6.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.85, 0.78, 0.5))),
        Transform::from_xyz(0.0, 0.02, -2.0),
    ));
```

- [ ] **Step 3: Apply extra friction while over sand**

Modify `ball_physics` in `src/main.rs` so the friction coefficient increases when the ball is inside any sand region. Replace `ball_physics` with:

```rust
fn ball_physics(
    time: Res<Time>,
    sand_q: Query<&Sand>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity) in &mut query {
        let p = transform.translation;
        let in_sand = sand_q.iter().any(|s| {
            p.x >= s.min.x && p.x <= s.max.x && p.z >= s.min.z && p.z <= s.max.z
        });
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
```

- [ ] **Step 4: Run and watch**

Run: `cargo run`
Expected: rolling the ball through the tan patch slows it sharply compared to the green. Close the window.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs src/components.rs
git commit -m "feat: sand traps apply extra friction"
```

---

### Task 16: Water / out-of-bounds reset with penalty (TDD)

**Files:**
- Modify: `src/physics.rs`, `src/components.rs`, `src/state.rs`, `src/main.rs`

- [ ] **Step 1: Write the failing test for `is_out_of_bounds`**

Add to `src/physics.rs` (above the test module):

```rust
/// True when a position is outside the play rectangle (half-extents from origin).
pub fn is_out_of_bounds(pos: Vec3, half_x: f32, half_z: f32) -> bool {
    pos.x.abs() > half_x || pos.z.abs() > half_z
}
```

Add tests inside the `tests` module:

```rust
    #[test]
    fn inside_bounds_is_not_oob() {
        assert!(!is_out_of_bounds(Vec3::new(1.0, 0.3, -2.0), 10.0, 10.0));
    }

    #[test]
    fn outside_bounds_is_oob() {
        assert!(is_out_of_bounds(Vec3::new(11.0, 0.3, 0.0), 10.0, 10.0));
    }
```

- [ ] **Step 2: Run the tests**

Run: `cargo test`
Expected: PASS (14 tests total).

- [ ] **Step 3: Track the ball's last resting position**

Append to `src/state.rs`:

```rust
/// The ball's last at-rest position, used to reset after water/out-of-bounds.
#[derive(Resource)]
pub struct LastRest(pub Vec3);

impl Default for LastRest {
    fn default() -> Self {
        Self(Vec3::new(0.0, 0.3, 6.0))
    }
}
```

In `src/main.rs`: `.init_resource::<LastRest>()` (add `use state::LastRest;`). In `ball_physics`, when the ball comes to rest, record its position. Inside the `if is_at_rest(...)` block, before `next_state.set(...)`, you'll set `LastRest`; to do that, add `mut last_rest: ResMut<LastRest>` to `ball_physics`'s parameters and set `last_rest.0 = transform.translation;` inside the rest block.

- [ ] **Step 4: Add a `Water` region and a reset system**

Append to `src/components.rs`:

```rust
/// A rectangular water hazard (X-Z bounds).
#[derive(Component)]
pub struct Water {
    pub min: Vec3,
    pub max: Vec3,
}
```

In `src/main.rs`, add `use components::Water;`, `use physics::is_out_of_bounds;`. In `setup`, spawn a blue water patch:

```rust
    // A blue water hazard off to the left.
    commands.spawn((
        Water { min: Vec3::new(-8.0, 0.0, -5.0), max: Vec3::new(-4.0, 0.0, -1.0) },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(4.0, 4.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 0.85))),
        Transform::from_xyz(-6.0, 0.02, -3.0),
    ));
```

Add a hazard-reset system that runs while the ball moves:

```rust
fn hazard_check(
    mut strokes: ResMut<Strokes>,
    last_rest: Res<LastRest>,
    water_q: Query<&Water>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ball_q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let Ok((mut t, mut v)) = ball_q.get_single_mut() else { return };
    let p = t.translation;
    let in_water = water_q.iter().any(|w| {
        p.x >= w.min.x && p.x <= w.max.x && p.z >= w.min.z && p.z <= w.max.z
    });
    if in_water || is_out_of_bounds(p, 10.0, 10.0) {
        t.translation = last_rest.0;
        v.0 = Vec3::ZERO;
        strokes.0 += 1; // penalty stroke
        next_state.set(GameState::Aiming);
    }
}
```

Register: `.add_systems(Update, hazard_check.run_if(in_state(GameState::BallMoving)))`

- [ ] **Step 5: Run and watch**

Run: `cargo run`
Expected: rolling into the blue water (or off the course) snaps the ball back to where it last rested, adds a penalty stroke (watch the HUD count jump), and returns you to aiming. Close the window.

- [ ] **Step 6: Commit**

```bash
git add src/main.rs src/physics.rs src/components.rs src/state.rs
git commit -m "feat: water/out-of-bounds reset with penalty stroke"
```

---

## Phase 6 — Multiple holes + total score

### Task 17: Course data — load a hole by index

**Files:**
- Modify: `src/course.rs` (new), `src/main.rs`

- [ ] **Step 1: Create a course module describing each hole**

Create `src/course.rs`:

```rust
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
        HoleLayout { ball_start: Vec3::new(0.0, 0.3, 6.0),  hole_pos: Vec3::new(0.0, 0.01, -6.0), hole_radius: 0.6 },
        HoleLayout { ball_start: Vec3::new(-6.0, 0.3, 6.0), hole_pos: Vec3::new(6.0, 0.01, -6.0), hole_radius: 0.6 },
        HoleLayout { ball_start: Vec3::new(6.0, 0.3, 7.0),  hole_pos: Vec3::new(-5.0, 0.01, -7.0), hole_radius: 0.5 },
    ]
}

/// Which hole is currently active and the running total.
#[derive(Resource, Default)]
pub struct Course {
    pub index: usize,
    pub total_strokes: u32,
}
```

- [ ] **Step 2: Spawn the ball and hole from the active layout**

In `src/main.rs`, add `mod course;`, `use course::{course, Course};`, and `.init_resource::<Course>()`. Replace the hard-coded ball and hole spawns in `setup` with values from `course()[0]`:

```rust
    let layout = &course()[0];
    // Ball
    commands.spawn((
        Ball,
        Velocity::default(),
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(layout.ball_start),
    ));
    // Hole
    commands.spawn((
        Hole { radius: layout.hole_radius },
        Mesh3d(meshes.add(Cylinder::new(layout.hole_radius, 0.02))),
        MeshMaterial3d(materials.add(Color::srgb(0.05, 0.05, 0.05))),
        Transform::from_translation(layout.hole_pos),
    ));
```

Also set `LastRest` default to match (initialize the resource after spawn, or set `last_rest.0 = layout.ball_start` in a startup system). Simplest: in `setup`, after inserting, do nothing special — `LastRest::default()` already matches hole 0's start.

- [ ] **Step 3: Run and watch**

Run: `cargo run`
Expected: identical to before — hole 0 loads from the data table. (Pure refactor; confirm nothing regressed.) Close the window.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/course.rs
git commit -m "refactor: load the first hole from course data"
```

---

### Task 18: Advance hole-to-hole + final total

**Files:**
- Modify: `src/components.rs`, `src/state.rs`, `src/ui.rs`, `src/main.rs`

- [ ] **Step 1: Mark per-hole entities so they can be despawned**

Append to `src/components.rs`:

```rust
/// Tags entities that belong to the current hole and should be cleared on advance.
#[derive(Component)]
pub struct HoleEntity;
```

Add `HoleEntity` to the Ball and Hole spawns from Task 17 (add it as a component in each tuple).

- [ ] **Step 2: Add a `GameOver` state**

In `src/state.rs`, add to `GameState`:

```rust
    GameOver,
```

- [ ] **Step 3: On HoleComplete entry, bank strokes and schedule the next hole**

In `src/main.rs`, modify `ui::show_win` flow: when entering `HoleComplete`, add the current `strokes` to `course.total_strokes`. Then a key press advances. Add this system and register it with `.add_systems(Update, advance_hole.run_if(in_state(GameState::HoleComplete)))`:

```rust
fn advance_hole(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut course_res: ResMut<Course>,
    mut strokes: ResMut<Strokes>,
    mut last_rest: ResMut<LastRest>,
    mut next_state: ResMut<NextState<GameState>>,
    hole_entities: Query<Entity, With<components::HoleEntity>>,
    banner_q: Query<Entity, With<ui::WinBanner>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    // Despawn old hole + win banner.
    for e in &hole_entities { commands.entity(e).despawn(); }
    for e in &banner_q { commands.entity(e).despawn(); }

    course_res.index += 1;
    let layouts = course::course();
    if course_res.index >= layouts.len() {
        next_state.set(GameState::GameOver);
        return;
    }
    let layout = &layouts[course_res.index];
    strokes.0 = 0;
    last_rest.0 = layout.ball_start;

    commands.spawn((
        Ball, Velocity::default(), components::HoleEntity,
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(layout.ball_start),
    ));
    commands.spawn((
        Hole { radius: layout.hole_radius }, components::HoleEntity,
        Mesh3d(meshes.add(Cylinder::new(layout.hole_radius, 0.02))),
        MeshMaterial3d(materials.add(Color::srgb(0.05, 0.05, 0.05))),
        Transform::from_translation(layout.hole_pos),
    ));
    next_state.set(GameState::Aiming);
}
```

- [ ] **Step 4: Bank the hole's strokes when it completes**

Add a small system on `OnEnter(GameState::HoleComplete)` that adds the hole's strokes to the total:

```rust
fn bank_strokes(strokes: Res<Strokes>, mut course_res: ResMut<Course>) {
    course_res.total_strokes += strokes.0;
}
```

Register: `.add_systems(OnEnter(GameState::HoleComplete), bank_strokes)` (alongside `ui::show_win`). Update `ui::show_win` text to prompt the next hole — change its `Text::new(...)` to:

```rust
        Text::new(format!("Hole done in {} strokes!\nPress Space for next hole", strokes.0)),
```

- [ ] **Step 5: Add a game-over screen showing the total**

Append to `src/ui.rs`:

```rust
use crate::course::Course;

#[derive(Component)]
pub struct GameOverBanner;

pub fn show_game_over(mut commands: Commands, course_res: Res<Course>) {
    commands.spawn((
        GameOverBanner,
        Text::new(format!("Course complete!\nTotal: {} strokes", course_res.total_strokes)),
        TextFont { font_size: 44.0, ..default() },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(28.0),
            ..default()
        },
    ));
}
```

Register: `.add_systems(OnEnter(GameState::GameOver), ui::show_game_over)`.

- [ ] **Step 6: Run and watch (full game)**

Run: `cargo run`
Expected: sink hole 1 → "Press Space for next hole" → hole 2 loads with the ball/hole in new spots and strokes reset to 0 → sink it → hole 3 → after the last hole, a "Course complete! Total: N strokes" screen. Close the window.

- [ ] **Step 7: Commit**

```bash
git add src/main.rs src/components.rs src/state.rs src/ui.rs
git commit -m "feat: advance through multiple holes with a final total score"
```

**✅ Full game complete.**

---

## Final verification

- [ ] Run `cargo test` — all pure-physics tests pass (14 tests).
- [ ] Run `cargo run` and play all holes start to finish without a crash.
- [ ] `git log --oneline` shows one commit per working step.
- [ ] Push: `git push` (the GitHub repo already exists as `origin`).

## Notes for the implementer

- If a Bevy API call doesn't compile, the installed version differs from the 0.15+ style assumed here. Read the exact compiler error, check the Bevy migration guide for your version, and adapt the *spawn/component syntax* — the task structure and the `physics.rs` math stay the same.
- If you get stuck in a compile loop on the same error 2–3 times, stop, `git reset --hard` to the last working commit, and try a smaller change or a clearer prompt.
- These tuning numbers (friction 1.2, max_speed 12.0, restitution 0.7, slope tilt) are starting points — adjust them by feel once the mechanics work.
