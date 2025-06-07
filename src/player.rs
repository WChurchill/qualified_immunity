use avian2d::prelude::*;
use bevy::prelude::*;

use crate::enemy::{Hostile, VirusAttached};
use crate::movement::{Speed, Velocity};
use crate::player_attack::PlayerActionParams;
use crate::schedule::InGameSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_velocity.in_set(InGameSet::UserInput));
        app.add_systems(Update, update_camera.in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct WhiteBloodCellBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub velocity: Velocity,
    pub speed: Speed,
    pub collider: Collider,
    pub colliding_entities: CollidingEntities,
    pub collision_events: CollisionEventsEnabled,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub action_params: PlayerActionParams,
    pub white_blood_cell_bundle: WhiteBloodCellBundle,
}

fn update_velocity(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut Velocity, &Speed), With<Player>>,
) {
    for (mut velocity, speed) in &mut players {
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

        velocity.value = velocity.value.normalize_or_zero() * speed.current;
    }
}

pub fn handle_virus_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    enemies: Query<Entity, (With<Hostile>, Without<VirusAttached>)>,
) {
    let Ok(entity) = enemies.get(trigger.collider) else {
        return;
    };

    commands.entity(entity).despawn();
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
