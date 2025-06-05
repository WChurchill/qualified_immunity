use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::enemy::create_virus;
use crate::host::Host;
use crate::movement::Velocity;
use crate::player::Player;
use crate::player::PlayerBundle;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_enemies, spawn_walls));

        app.insert_resource(ClearColor(Color::oklcha(0.72, 0.15, 15.8, 1.0)));
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(PlayerBundle {
        sprite: Sprite {
            image: asset_server.load("white_blood_cell.png"),
            custom_size: Some(Vec2::splat(40.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        velocity: Velocity::new(Vec3::ZERO),
        marker: Player,
        collider: Collider::circle(20.0),
    });
}

#[derive(Component, Clone)]
pub struct Host {
    pub seconds_to_death: f32,
    pub rate_of_decay: f32,
    pub num_offspring: i32,
}

impl Host {
    pub fn new(seconds_to_death: f32, num_offspring: i32) -> Self {
        Host {
            seconds_to_death: seconds_to_death,
            rate_of_decay: 0.0,
            num_offspring: num_offspring,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct WallCellBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub host: Host,
}

fn random_rotate_cell(bundle: &mut WallCellBundle, rng: &mut ThreadRng) {
    let dir: f32 = rng.random_range(0..4) as f32 * PI / 2.0;
    bundle.transform.rotate_z(dir);
    bundle.sprite.flip_x = rng.random_bool(0.5);
    bundle.sprite.flip_y = rng.random_bool(0.5);
}

fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    const BOX_GRID_WIDTH: i32 = 10;
    const BOX_GRID_HEIGHT: i32 = 10;

    const X_OFFSET: f32 = 300.;
    const Y_OFFSET: f32 = 0.;

    const HITBOX_WIDTH: f32 = 40.;
    let template = WallCellBundle {
        sprite: Sprite {
            image: asset_server.load("wall_cell.png"),
            custom_size: Some(Vec2::splat(40.)),
            ..default()
        },
        transform: Transform::IDENTITY,
        collider: Collider::rectangle(HITBOX_WIDTH, HITBOX_WIDTH),
        host: Host::new(20.0, 4),
    };

    let mut rng: ThreadRng = rand::rng();

    for i in 0..BOX_GRID_HEIGHT {
        for j in 0..BOX_GRID_WIDTH {
            if rng.random_bool(0.05) {
                continue;
            }

            let mut wall_cell = template.clone();
            wall_cell.transform.translation.x = j as f32 * HITBOX_WIDTH + X_OFFSET;
            wall_cell.transform.translation.y = i as f32 * HITBOX_WIDTH + Y_OFFSET;

            // Randomize the appearance to make them all look different.
            random_rotate_cell(&mut wall_cell, &mut rng);

            commands.spawn(wall_cell);
        }
    }
}

const INITIAL_ENEMIES: i32 = 100;
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
