use bevy::prelude::*;
use rand::prelude::*;

use crate::movement::Velocity;
use crate::player::Player;
use crate::player::PlayerBundle;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);

        app.insert_resource(ClearColor(Color::oklcha(0.72, 0.15, 15.8, 1.0)));
    }
}

fn init_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(PlayerBundle {
        sprite: Sprite::from_image(asset_server.load("white_blood_cell.png")),
        transform: Transform::from_scale(Vec3::splat(0.05)),
        velocity: Velocity::new(Vec3::ZERO),
        marker: Player,
    });

    const WINDOW_HEIGHT: f32 = 600.0;
    const WINDOW_WIDTH: f32 = 800.0;

    for _ in 0..100 {
        let mut rng = rand::rng();
        let position = Vec2::new(
            rng.random_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
            rng.random_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
        );

        commands.spawn(VirusBundle {
            sprite: Sprite::from_image(asset_server.load("virus.png")),
            transform: Transform {
                translation: position.extend(0.0),
                scale: Vec3::splat(0.1),
                ..Default::default()
            },
        });
    }
}

#[derive(Bundle)]
pub struct VirusBundle {
    pub sprite: Sprite,
    pub transform: Transform,
}
