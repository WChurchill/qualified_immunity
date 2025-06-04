use crate::movement::Velocity;
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::schedule::InGameSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_velocity.in_set(InGameSet::UserInput));
        app.add_systems(Update, update_camera.in_set(InGameSet::EntityUpdates));
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
    pub collider: Collider,
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

        let mut movement_speed = MOVEMENT_SPEED;
        if keyboard.pressed(KeyCode::Space) {
            movement_speed *= 2.0;
        }

        velocity.value = velocity.value.normalize_or_zero() * movement_speed;
    }
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let target = player.translation.xy().extend(camera.translation.z);

    const CAMERA_DECAY: f32 = 0.99;
    camera
        .translation
        .smooth_nudge(&target, CAMERA_DECAY, time.delta_secs());
}
