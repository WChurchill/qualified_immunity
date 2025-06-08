use bevy::prelude::*;

use crate::level::EnemySpawner;
use crate::player_attack::{
    BoostBar, DuplicationBar, DuplicationCharge, PlayerChargingGUI, CHARGEBAR_WIDTH,
};
use crate::schedule::InGameSet;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud);
        app.add_systems(Update, update_wave_text.in_set(InGameSet::EntityUpdates));
    }
}

fn setup_hud(mut commands: Commands) {
    commands.insert_resource(PlayerChargingGUI {
        current_boost_level: 0.,
        max_boost_level: 2.,
    });

    commands.insert_resource(DuplicationCharge {
        current_progress: 0.,
        max_progress: 0.5, //4.0,
    });

    commands.spawn((
        Text::new("Level: "),
        children![(TextSpan::default(), WaveText)],
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::ColumnReverse,
            row_gap: Val::Px(5.0),
            bottom: Val::Px(5.),
            left: Val::Px(10.),
            width: Val::Px(CHARGEBAR_WIDTH),
            ..default()
        },
        children![
            (
                Node {
                    position_type: PositionType::Relative,
                    height: Val::Px(25.0),
                    ..default()
                },
                Outline {
                    width: Val::Px(4.),
                    color: Color::WHITE,
                    offset: Val::Px(0.0),
                },
                children![
                    (
                        BoostBar,
                        Node {
                            left: Val::Px(0.0),
                            overflow: Overflow::visible(),
                            ..default()
                        },
                        BackgroundColor(Color::Oklcha(Oklcha::lch(0.5, 0.5, 0.5))),
                    ),
                    (
                        Text::new("Hold space then release to boost "),
                        Node {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                    )
                ],
            ),
            (
                Node {
                    position_type: PositionType::Relative,
                    height: Val::Px(25.0),
                    ..default()
                },
                Outline {
                    width: Val::Px(4.0),
                    color: Color::WHITE,
                    offset: Val::Px(0.0),
                },
                children![
                    (
                        DuplicationBar,
                        Node {
                            left: Val::Px(0.0),
                            overflow: Overflow::visible(),
                            ..default()
                        },
                        BackgroundColor(Color::Oklcha(Oklcha::lch(0.44, 0.06, 245.0))),
                    ),
                    (
                        Text::new("Hold shift to self-replicate"),
                        Node {
                            position_type: PositionType::Absolute,
                            ..default()
                        }
                    ),
                ],
            ),
            (
                Node { ..default() },
                Outline {
                    width: Val::Px(4.0),
                    color: Color::WHITE,
                    offset: Val::Px(0.0),
                },
                Text::new("Move with arrow keys"),
            ),
        ],
    ));
}

#[derive(Component)]
struct WaveText;

fn update_wave_text(
    spawner: Query<&EnemySpawner>,
    mut wave_text: Query<&mut TextSpan, With<WaveText>>,
) {
    let query_result = spawner.single();
    for mut text in &mut wave_text {
        match query_result {
            Ok(unique_spawner) => {
                text.0 = format!("{}", unique_spawner.wave - 1);
            }
            Err(_) => {
                println!("Too many spawners!");
            }
        };
    }
}
