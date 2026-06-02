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
