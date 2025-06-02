use crate::movement::Velocity;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_velocity);
    }
}

const MOVEMENT_SPEED: f32 = 150.;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub velocity: Velocity,
    pub marker: Player,
}

fn update_velocity(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
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
