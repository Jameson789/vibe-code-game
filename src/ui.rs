use bevy::prelude::*;
use crate::course::Course;
use crate::state::{AimState, Strokes};

/// Marker for the heads-up text.
#[derive(Component)]
pub struct HudText;

/// Spawn the HUD once at startup.
pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        HudText,
        Text::new("Strokes: 0\nPower: 50%"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
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

/// Marker for the win message.
#[derive(Component)]
pub struct WinBanner;

/// Show a centered win message when the hole is completed.
pub fn show_win(mut commands: Commands, strokes: Res<Strokes>) {
    commands.spawn((
        WinBanner,
        Text::new(format!(
            "Hole done in {} strokes!\nPress Space for next hole",
            strokes.0
        )),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(43.0),
            left: Val::Percent(28.0),
            ..default()
        },
    ));
}

/// Marker for the end-of-course screen.
#[derive(Component)]
pub struct GameOverBanner;

/// Show the final total when every hole is complete.
pub fn show_game_over(mut commands: Commands, course_res: Res<Course>) {
    commands.spawn((
        GameOverBanner,
        Text::new(format!(
            "Course complete!\nTotal: {} strokes",
            course_res.total_strokes
        )),
        TextFont {
            font_size: 44.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(28.0),
            ..default()
        },
    ));
}

/// Marker for the temporary hazard message.
#[derive(Component)]
pub struct PenaltyBanner;

/// Show a centered "water hazard" message during the penalty pause.
pub fn show_penalty(mut commands: Commands) {
    commands.spawn((
        PenaltyBanner,
        Text::new("Water hazard!  +1 stroke"),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.8, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(45.0),
            left: Val::Percent(30.0),
            ..default()
        },
    ));
}
