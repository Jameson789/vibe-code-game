# Design: Simple 3D Golf (Bevy + Rust)

**Date:** 2026-06-01
**Status:** Approved

## Context

This is a vibe-coding class exercise (SDEV378 / Applied AI – Vibe Coding). The
course rules (see the `0X-*.md` files copied into this folder) require:

- Build something **original** in a stack we can't read or write fluently (Rust + Bevy).
- **Start with the smallest possible MVP**, then add one feature at a time.
- **Always verify** by running and watching it — "it compiles" is not "it works".
- **Use Git and commit the moment something works.**

The agent (Claude) writes the Rust; the human steers.

## Concept

A behind-the-ball 3D golf game. The player aims, sets power, and swings to roll a
ball into a hole in as few strokes as possible. Original enough to avoid the
"agent has memorized the tutorial" trap (not Pong/Snake/Flappy), simple enough to
get an MVP on screen fast.

## Controls

| Input | Action |
|-------|--------|
| **← / →** | Rotate aim direction (chase camera swings with it) |
| **↑ / ↓** | Increase / decrease shot power (shown as a bar) |
| **Spacebar** | Swing — only allowed while the ball is at rest |

## Tech Stack

- **Rust** (latest stable, installed via rustup.rs).
- **Bevy** (latest stable, pinned by `cargo add bevy`) — pure code, no visual
  editor, which is the whole point of the course's engine choice.
- **No physics engine.** Custom lightweight physics (velocity + friction, wall
  reflection, downhill slope force). Keeps dependencies minimal, keeps each
  incremental step tiny, and avoids the notorious bevy_rapier/Bevy version-match
  compile loop. Rapier remains a fallback if custom math becomes too painful.
- **Git** — commit each working step.

## Architecture (Bevy ECS)

### Components
- `Ball` — the player's ball (a sphere mesh). Paired with `Velocity(Vec3)`.
- `Hole` — target position + radius on the ground.
- Course geometry — a ground plane; later `Wall`, `Slope`, and hazard entities
  (`Sand`, `Water`/out-of-bounds).

### Resources
- `AimState` — current aim yaw (radians) + current power (0.0–1.0).
- `Strokes` — stroke count for the current hole (and later, total).

### States (Bevy `States`)
- `Aiming` → `BallMoving` → `HoleComplete` → (later) `NextHole`.

### Systems
- `input_aim_power` (in `Aiming`): ←/→ adjust aim yaw, ↑/↓ adjust power (clamped
  0–1), Spacebar launches the ball (sets `Velocity` from aim + power, increments
  `Strokes`, transitions to `BallMoving`).
- `ball_physics` (in `BallMoving`): integrate position from velocity, apply
  rolling friction, stop the ball when its speed drops below a threshold →
  transition back to `Aiming`.
- `chase_camera`: place the camera behind the ball along the aim direction and
  look at the ball.
- `hole_check`: if the ball is within the hole radius and slow enough →
  `HoleComplete` (or advance to the next hole).
- *(later)* `wall_collision`: reflect velocity off wall surfaces.
- *(later)* `slope_force`: add downhill acceleration based on ground slope.
- *(later)* `hazard_check`: sand increases friction; water/out-of-bounds resets
  the ball to its last resting position with a stroke penalty.
- `ui`: display the power bar and stroke count; win screen on `HoleComplete`.

### Data Flow

```
Input ─▶ AimState ──swing──▶ Ball.Velocity ─▶ ball_physics integrates position
                                                   │
                          hole_check / hazard_check ─▶ state transitions
                                                   │
                              UI reads Strokes + current state
```

### File Structure
Start everything in a single `main.rs` for the bare MVP. As it grows, split into
focused modules — `ball`, `camera`, `input`, `physics`, `course`, `ui`, `state` —
so no single file does too much.

## Build Order (MVP-first, one tiny verified step at a time)

1. **Bare MVP** — window → lit ground plane + ball → aim/power/swing → ball rolls
   with friction and stops → static behind-ball camera.
2. **Chase camera** that rotates with aim, plus an aim-direction indicator line.
3. **Hole + sink detection + stroke counter & win screen.**
4. **Walls & slopes** — bounce off walls, roll downhill on slopes.
5. **Obstacles & hazards** — sand slows the ball; water/out-of-bounds resets the
   shot with a penalty.
6. **Multiple holes** — advance hole-to-hole, track total score.

## Win Condition

The ball settles inside the hole radius below a speed threshold → `HoleComplete`,
showing the stroke count. With multiple holes, advance to the next hole layout and
accumulate total strokes.

## Error Handling & Robustness

- Clamp power to 0.0–1.0.
- Block swinging while the ball is moving (state gate).
- Stuck-ball timeout to force a return to rest.
- Out-of-bounds / water resets the ball to its last resting position (configurable
  stroke penalty).
- Keep `cargo run` iteration fast in dev (consider Bevy dynamic linking feature).

## Testing & Verification

Per the course rules, verification is primarily **run-and-watch** at every
increment, with a Git commit on each working state. Pure-logic functions (physics
integration step, hole-distance check, velocity reflection) get small unit tests
so the math is pinned down independently of the visual run.
