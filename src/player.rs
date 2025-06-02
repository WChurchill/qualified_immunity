use crate::movement::Velocity;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, update_velocity);
    }
}

const MOVEMENT_SPEED: f32 = 150.;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    println!("spawn player");
    commands.spawn((
        Sprite::from_image(asset_server.load("white_blood_cell.png")),
        Transform::from_scale(Vec3::splat(0.05)),
        Velocity::new(Vec3::ZERO),
    ));
}

fn update_velocity(keyboard: Res<ButtonInput<KeyCode>>, mut players: Query<&mut Velocity>) {
    for mut velocity in &mut players {
        velocity.value = Vec3::ZERO;
        if keyboard.pressed(KeyCode::ArrowLeft) {
            velocity.value.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            velocity.value.x += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            velocity.value.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            velocity.value.y += 1.0;
        }

        velocity.value = velocity.value.normalize_or_zero() * MOVEMENT_SPEED;
    }
}
