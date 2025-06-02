use bevy::prelude::*;

use crate::movement::Velocity;
use crate::player::Player;
use crate::player::PlayerBundle;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
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
}
