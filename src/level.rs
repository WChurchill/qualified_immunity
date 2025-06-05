use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::enemy::create_virus;
use crate::host::{handle_infection, Host};
use crate::movement::{Speed, Velocity};
use crate::player::{handle_virus_collision, Player, PlayerBundle};
use crate::player_attack::{
    BoostText, DuplicationChargingGUI, DuplicationText, PlayerActionParams, PlayerChargingGUI,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_enemies, spawn_walls));

        app.insert_resource(ClearColor(Color::oklcha(0.72, 0.15, 15.8, 1.0)));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let rect = meshes.add(Rectangle::new(200., 50.));
    commands
        .spawn((
            Text::new("Hold space to boost "),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.),
                left: Val::Px(10.),
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            BoostText,
            Mesh2d(rect),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        ));

    commands.insert_resource(PlayerChargingGUI {
        current_boost_level: 0.,
        max_boost_level: 2.,
    });

    commands.insert_resource(DuplicationChargingGUI {
        current_progress: 0.,
        max_progress: 3.0,
    });

    commands
        .spawn((
            PlayerBundle {
                sprite: Sprite {
                    image: asset_server.load("white_blood_cell.png"),
                    custom_size: Some(Vec2::splat(40.)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 0.),
                velocity: Velocity::new(Vec3::ZERO),
                speed: Speed::new(150.),
                marker: Player,
                collider: Collider::circle(20.0),
                colliding_entities: CollidingEntities::default(),
                action_params: PlayerActionParams {
                    boosted_speed: 300.,
                    remaining_secs: 0.,
                    extra_seconds_per_boost_level: 0.1,
                    extra_speed_per_boost_level: 50.,
                },
            },
            CollisionEventsEnabled,
        ))
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
    const BOX_GRID_WIDTH: i32 = 10;
    const BOX_GRID_HEIGHT: i32 = 10;

    const X_OFFSET: f32 = 0.;
    const Y_OFFSET: f32 = 0.;
    const INTERCELL_GAP: f32 = 20.;

    const HITBOX_WIDTH: f32 = 35.;
    let template = WallCellBundle {
        sprite: Sprite {
            image: asset_server.load("wall_cell.png"),
            custom_size: Some(Vec2::splat(40.)),
            ..default()
        },
        transform: Transform::IDENTITY,
        collider: Collider::rectangle(HITBOX_WIDTH, HITBOX_WIDTH),
        host: Host,
    };

    let mut rng: ThreadRng = rand::rng();

    for i in 0..BOX_GRID_HEIGHT {
        for j in 0..BOX_GRID_WIDTH {
            if rng.random_bool(0.05) {
                continue;
            }

            let mut wall_cell = template.clone();
            wall_cell.transform.translation.x =
                j as f32 * (HITBOX_WIDTH + INTERCELL_GAP) + X_OFFSET;
            wall_cell.transform.translation.y =
                i as f32 * (HITBOX_WIDTH + INTERCELL_GAP) + Y_OFFSET;

            // Randomize the appearance to make them all look different.
            random_rotate_cell(&mut wall_cell, &mut rng);

            commands
                .spawn((wall_cell, CollisionEventsEnabled))
                .observe(handle_infection);
        }
    }
}

const INITIAL_ENEMIES: i32 = 5;
fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    const WINDOW_HEIGHT: f32 = 600.0;
    const WINDOW_WIDTH: f32 = 800.0;

    for _ in 0..INITIAL_ENEMIES {
        let mut rng = rand::rng();
        let position = Vec2::new(
            rng.random_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
            rng.random_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
        );

        let random_direction = Vec2::from_angle(rng.random_range(0.0..2.0 * PI));
        commands.spawn(create_virus(&asset_server, random_direction, position));
    }
}
