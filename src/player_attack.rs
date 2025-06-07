use avian2d::prelude::*;
use bevy::prelude::*;

use crate::movement::{Speed, Velocity};
use crate::player::{handle_virus_collision, Player};
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
            (display_boost, display_multiply, player_boost).in_set(InGameSet::EntityUpdates),
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
pub struct DuplicationCharge {
    pub current_progress: f32,
    pub max_progress: f32,
}

#[derive(Component)]
pub struct DuplicationBar;

const CHARGE_MULTIPLY_KEYBINDING: KeyCode = KeyCode::ShiftLeft;
fn charge_multiply(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut charging: ResMut<DuplicationCharge>,
    mut players: Query<(&mut Speed, &Transform), With<Player>>,
) {
    if keyboard.pressed(CHARGE_MULTIPLY_KEYBINDING) {
        charging.current_progress += time.delta_secs();

        if charging.current_progress >= charging.max_progress {}

        for (mut speed, transform) in &mut players {
            if charging.current_progress < charging.max_progress {
                // Incur some cost to duplicating oneself.
                speed.current = speed.default / 2.0;
                continue;
            }

            charging.current_progress = 0.0;

            commands
                .spawn((
                    Sprite {
                        image: asset_server.load("white_blood_cell.png"),
                        custom_size: Some(Vec2::splat(40.0)),
                        ..default()
                    },
                    Transform::from_translation(transform.translation),
                    Velocity::new(Vec3::ZERO),
                    Speed::new(75.0),
                    Collider::circle(20.0),
                    CollidingEntities::default(),
                    CollisionEventsEnabled,
                ))
                .observe(handle_virus_collision);
        }
    } else if keyboard.just_released(CHARGE_MULTIPLY_KEYBINDING) {
        charging.current_progress = 0.0;
    }
}

fn display_multiply(
    charging: Res<DuplicationCharge>,
    mut query: Query<&mut Node, With<DuplicationBar>>,
) {
    for mut node in &mut query {
        let width = charging.current_progress * CHARGEBAR_WIDTH / charging.max_progress;
        node.width = Val::Px(width);
    }
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

#[derive(Component)]
pub struct BoostBar;

pub const CHARGEBAR_WIDTH: f32 = 400.0;

fn display_boost(
    charging: Res<PlayerChargingGUI>,
    mut bar_query: Query<&mut Node, With<BoostBar>>,
) {
    for mut node in &mut bar_query {
        let width = charging.current_boost_level * CHARGEBAR_WIDTH / charging.max_boost_level;
        node.width = Val::Px(width);
    }
}
