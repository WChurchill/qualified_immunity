use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::enemy::{create_virus, Hostile};
use crate::host::{handle_infection, Host};
use crate::movement::{Speed, Velocity};
use crate::player::{handle_virus_collision, Player, PlayerBundle, WhiteBloodCellBundle};
use crate::player_attack::{
    BoostBar, DuplicationBar, DuplicationCharge, PlayerActionParams, PlayerChargingGUI,
    CHARGEBAR_WIDTH,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, setup_enemy_spawner, spawn_walls));
        app.add_systems(Update, (spawn_enemies, update_wave_text));
        app.insert_resource(ClearColor(Color::oklcha(0.72, 0.15, 15.8, 1.0)));
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.insert_resource(PlayerChargingGUI {
        current_boost_level: 0.,
        max_boost_level: 2.,
    });

    commands.insert_resource(DuplicationCharge {
        current_progress: 0.,
        max_progress: 4.0,
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

    commands
        .spawn((PlayerBundle {
            marker: Player,
            action_params: PlayerActionParams {
                boosted_speed: 300.,
                remaining_secs: 0.,
                extra_seconds_per_boost_level: 0.1,
                extra_speed_per_boost_level: 50.,
            },
            white_blood_cell_bundle: WhiteBloodCellBundle {
                sprite: Sprite {
                    image: asset_server.load("white_blood_cell.png"),
                    custom_size: Some(Vec2::splat(40.)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 0.),
                velocity: Velocity::new(Vec3::ZERO),
                speed: Speed::new(150.),
                collider: Collider::circle(20.0),
                colliding_entities: CollidingEntities::default(),
                collision_events: CollisionEventsEnabled,
            },
        },))
        .observe(handle_virus_collision);
}

#[derive(Bundle, Clone)]
pub struct WallCellBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub host: Host,
}

fn random_rotate_cell(bundle: &mut WallCellBundle, rng: &mut ThreadRng) {
    bundle.sprite.flip_x = rng.random_bool(0.5);
    bundle.sprite.flip_y = rng.random_bool(0.5);
}

fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    const BOX_GRID_WIDTH: i32 = 5;
    const BOX_GRID_HEIGHT: i32 = 5;

    const INTERCELL_GAP: f32 = 50.;

    const HITBOX_WIDTH: f32 = 35.;
    const SPRITE_WIDTH: f32 = 40.0;

    let x_offset: f32 =
        -((BOX_GRID_WIDTH - 1) as f32 * (INTERCELL_GAP + SPRITE_WIDTH) + SPRITE_WIDTH) / 2.0;
    let y_offset: f32 =
        -((BOX_GRID_HEIGHT - 1) as f32 * (INTERCELL_GAP + SPRITE_WIDTH) + SPRITE_WIDTH) / 2.0;

    let template = WallCellBundle {
        sprite: Sprite {
            image: asset_server.load("wall_cell.png"),
            custom_size: Some(Vec2::splat(SPRITE_WIDTH)),
            ..default()
        },
        transform: Transform::IDENTITY,
        collider: Collider::rectangle(HITBOX_WIDTH, HITBOX_WIDTH),
        host: Host,
    };

    let mut rng: ThreadRng = rand::rng();

    for i in 0..BOX_GRID_HEIGHT {
        for j in 0..BOX_GRID_WIDTH {
            if rng.random_bool(0.7) {
                continue;
            }

            let mut wall_cell = template.clone();
            wall_cell.transform.translation.x =
                j as f32 * (HITBOX_WIDTH + INTERCELL_GAP) + x_offset;
            wall_cell.transform.translation.y =
                i as f32 * (HITBOX_WIDTH + INTERCELL_GAP) + y_offset;

            // Randomize the appearance to make them all look different.
            random_rotate_cell(&mut wall_cell, &mut rng);

            commands
                .spawn((wall_cell, CollisionEventsEnabled))
                .observe(handle_infection);
        }
    }
}

#[derive(Component)]
struct EnemySpawner {
    radius: f32,
    cluster_radius: f32,
    wave: i32,
    timer_secs: f32,
}

fn setup_enemy_spawner(mut commands: Commands) {
    commands.spawn(EnemySpawner {
        radius: 1000.0,
        cluster_radius: 200.0,
        wave: 1,
        timer_secs: 0.0,
    });
}

const SECONDS_BETWEEN_WAVES: f32 = 3.0;

fn spawn_enemies(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut EnemySpawner>,
    remaining_enemies: Query<&Hostile>,
) {
    let Ok(mut enemy_spawner) = query.single_mut() else {
        println!("Expected unique enemy spawner");
        return;
    };

    if !remaining_enemies.is_empty() {
        return;
    }

    if enemy_spawner.timer_secs > 0.0 {
        enemy_spawner.timer_secs -= time.delta_secs();
        return;
    }

    let mut rng = rand::rng();

    let cluster_origin = enemy_spawner.radius * Vec2::from_angle(rng.random_range(0.0..2.0 * PI));

    let num_enemies = (enemy_spawner.wave as f32 + 1.5).powf(2.0);

    for _ in 0..num_enemies.ceil() as i32 {
        // Don't sample the circle uniformly it probably looks better
        // with more viruses near the center.
        let individual_offset = rng.random_range(0.0..enemy_spawner.cluster_radius)
            * Vec2::from_angle(rng.random_range(0.0..2.0 * PI));

        let random_direction = Vec2::from_angle(rng.random_range(0.0..2.0 * PI));
        commands.spawn(create_virus(
            &asset_server,
            random_direction,
            cluster_origin + individual_offset,
        ));
    }

    enemy_spawner.timer_secs = SECONDS_BETWEEN_WAVES;
    enemy_spawner.wave += 1;
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
                text.0 = format!("{}", unique_spawner.wave);
            }
            Err(_) => {
                println!("Too many spawners!");
            }
        };
    }
}
