use bevy::prelude::*;

use crate::movement::{Speed, Velocity};
use crate::player::Player;
use crate::schedule::InGameSet;

pub struct PlayerAttackPlugin;

impl Plugin for PlayerAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (charge_attack, charge_multiply).in_set(InGameSet::UserInput),
        );
        app.add_systems(
            Update,
            (display_boost, player_boost).in_set(InGameSet::EntityUpdates),
        );
    }
}

#[derive(Component)]
pub struct PlayerActionParams {
    pub boosted_speed: f32,
    pub remaining_secs: f32,
    pub extra_seconds_per_boost_level: f32,
    pub extra_speed_per_boost_level: f32,
}

#[derive(Resource)]
pub struct PlayerChargingGUI {
    pub current_boost_level: f32,
    pub max_boost_level: f32,
}

#[derive(Component)]
pub struct BoostText;

#[derive(Component)]
pub struct DuplicationText;

const BOOSTING_BASE_SPEED: f32 = 300.;
const BOOSTING_BASE_SECS: f32 = 0.3;

fn charge_attack(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut charging: ResMut<PlayerChargingGUI>,
    mut players: Query<(&mut Velocity, &mut PlayerActionParams), With<Player>>,
) {
    if keyboard.pressed(KeyCode::Space) {
        charging.current_boost_level += time.delta_secs();
        charging.current_boost_level =
            f32::min(charging.current_boost_level, charging.max_boost_level);
    } else if keyboard.just_released(KeyCode::Space) {
        for (_, mut params) in &mut players {
            params.remaining_secs = BOOSTING_BASE_SECS
                + params.extra_seconds_per_boost_level * charging.current_boost_level;
            params.boosted_speed = BOOSTING_BASE_SPEED
                + params.extra_speed_per_boost_level * charging.current_boost_level;
            charging.current_boost_level = 0.;
        }
    }
}

#[derive(Resource)]
pub struct DuplicationChargingGUI {
    pub current_progress: f32,
    pub max_progress: f32,
}

const CHARGE_MULTIPLY_KEYBINDING: KeyCode = KeyCode::ShiftLeft;
fn charge_multiply(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut charging: ResMut<DuplicationChargingGUI>,
    mut players: Query<&mut Speed, With<Player>>,
) {
    if keyboard.pressed(CHARGE_MULTIPLY_KEYBINDING) {
        charging.current_progress += time.delta_secs();
        charging.current_progress = f32::min(charging.current_progress, charging.max_progress);
        for mut speed in players.iter_mut() {
            speed.current = speed.default / 2.0;
        }
    } else if keyboard.just_released(CHARGE_MULTIPLY_KEYBINDING) {
        charging.current_progress = 0.0;
    }
}

fn display_multiply(
    charging: Res<DuplicationChargingGUI>,
    mut query: Query<(&mut TextSpan, &mut Mesh2d), With<DuplicationText>>,
) {
}

fn player_boost(
    time: Res<Time>,
    mut players: Query<(&mut Speed, &mut PlayerActionParams), With<Player>>,
) {
    for (mut speed, mut action_params) in &mut players {
        if action_params.remaining_secs < 0. {
            speed.current = speed.default;
            continue;
        }
        speed.current = action_params.boosted_speed;
        action_params.remaining_secs -= time.delta_secs();
    }
}

fn display_boost(
    charging: Res<PlayerChargingGUI>,
    mut query: Query<(&mut TextSpan, &mut Mesh2d), With<BoostText>>,
) {
    for (mut span, _) in &mut query {
        **span = format!(
            "{:.2} / {:.2}",
            charging.current_boost_level, charging.max_boost_level
        );
    }
}
