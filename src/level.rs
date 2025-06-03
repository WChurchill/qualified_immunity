use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::collision::Hitbox;
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
        sprite: Sprite::from_image(asset_server.load("white_blood_cell.png")),
        transform: Transform::from_scale(Vec3::splat(0.2)),
        velocity: Velocity::new(Vec3::ZERO),
        marker: Player,
        hitbox: Hitbox::Circle(Circle { radius: 20.0 }),
    });

    const WINDOW_HEIGHT: f32 = 600.0;
    const WINDOW_WIDTH: f32 = 800.0;

    for _ in 0..100 {
        let mut rng = rand::rng();
        let position = Vec2::new(
            rng.random_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
            rng.random_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
        );

        let random_direction = Vec2::from_angle(rng.random_range(0.0..2.0 * PI));
        const VIRUS_SPEED: f32 = 20.0;
        commands.spawn(VirusBundle {
            sprite: Sprite::from_image(asset_server.load("virus.png")),
            transform: Transform {
                translation: position.extend(0.0),
                scale: Vec3::splat(0.1),
                ..Default::default()
            },
            hitbox: Hitbox::Rectangle(Rectangle::new(15., 30.)),
            velocity: Velocity {
                value: random_direction.extend(0.) * VIRUS_SPEED,
            },
        });
    }
}

#[derive(Bundle, Clone)]
pub struct WallCellBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub hitbox: Hitbox,
}

fn random_rotate_cell(bundle: &mut WallCellBundle, rng: &mut ThreadRng) {
    let dir: f32 = rng.random_range(0..4) as f32 * PI / 2.0;
    bundle.transform.rotate_z(dir);
    bundle.sprite.flip_x = rng.random_bool(0.5);
    bundle.sprite.flip_y = rng.random_bool(0.5);
}

fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    const BOX_GRID_WIDTH: i32 = 30;

    const HITBOX_WIDTH: f32 = 40.;
    let template = WallCellBundle {
        sprite: Sprite::from_image(asset_server.load("wall_cell.png")),
        transform: Transform::from_scale(Vec3::splat(0.2)),
        hitbox: Hitbox::Rectangle(Rectangle::from_length(HITBOX_WIDTH)),
    };

    let mut rng: ThreadRng = rand::rng();

    for i in 0..BOX_GRID_WIDTH {
        let mut wall_cell = template.clone();
        wall_cell.transform.translation.x = i as f32 * HITBOX_WIDTH;
        wall_cell.transform.translation.y = 200.0;

        // Randomize the appearance to make them all look different.
        random_rotate_cell(&mut wall_cell, &mut rng);

        commands.spawn(wall_cell);
    }
}

#[derive(Bundle)]
pub struct VirusBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub hitbox: Hitbox,
    pub velocity: Velocity,
}

fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    const WINDOW_HEIGHT: f32 = 600.0;
    const WINDOW_WIDTH: f32 = 800.0;

    for _ in 0..100 {
        let mut rng = rand::rng();
        let position = Vec2::new(
            rng.random_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
            rng.random_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
        );

        let random_direction = Vec2::from_angle(rng.random_range(0.0..2.0 * PI));
        const VIRUS_SPEED: f32 = 20.0;
        commands.spawn(VirusBundle {
            sprite: Sprite::from_image(asset_server.load("virus.png")),
            transform: Transform {
                translation: position.extend(0.0),
                scale: Vec3::splat(0.1),
                ..Default::default()
            },
            hitbox: Hitbox::Rectangle(Rectangle::new(15., 30.)),
            velocity: Velocity {
                value: random_direction.extend(0.) * VIRUS_SPEED,
            },
        });
    }
}
