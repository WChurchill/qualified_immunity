use avian2d::prelude::*;
use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::host::Host;
use crate::movement::{Speed, TurnSpeed, Velocity};
use crate::schedule::InGameSet;

#[derive(Component)]
pub enum Hostile {
    // Should only attack once and then die.
    InfectThenDie,
}

#[derive(Component)]
pub struct Targeting(pub Entity);

#[derive(Component)]
pub struct VirusAttached;

#[derive(Bundle)]
pub struct VirusBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub velocity: Velocity,
    pub speed: Speed,
    pub turn_speed: TurnSpeed,
    pub enemy_class: Hostile,
    pub colliding_entities: CollidingEntities,
}

const VIRUS_SPRITE_FILEPATH: &'static str = "virus.png";
pub const VIRUS_HITBOX: (f32, f32) = (11.0, 8.0);
pub const VIRUS_SPRITE_SIZE: Vec2 = Vec2::splat(20.);

pub fn create_virus(
    asset_server: &Res<AssetServer>,
    direction: Vec2,
    position: Vec2,
) -> VirusBundle {
    VirusBundle {
        sprite: Sprite {
            image: asset_server.load(VIRUS_SPRITE_FILEPATH),
            custom_size: Some(VIRUS_SPRITE_SIZE),
            flip_y: true,
            ..default()
        },
        transform: Transform {
            translation: position.extend(0.),
            ..default()
        },
        collider: Collider::capsule_endpoints(
            VIRUS_HITBOX.0 / 2.0,
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, -3.0),
        ),
        velocity: Velocity {
            value: direction.extend(0.) * VIRUS_SPEED,
        },
        speed: Speed {
            current: VIRUS_SPEED,
            default: VIRUS_SPEED,
        },
        turn_speed: TurnSpeed(1.0),
        enemy_class: Hostile::InfectThenDie,
        colliding_entities: CollidingEntities::default(),
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (set_target, unset_nonexisting_target).in_set(InGameSet::EntityUpdates),
        );
        app.add_systems(Update, set_velocity.in_set(InGameSet::EntityUpdates));
    }
}

fn set_target(
    mut commands: Commands,
    mut viruses: Query<
        Entity,
        (
            With<Velocity>,
            With<Transform>,
            With<TurnSpeed>,
            With<Hostile>,
            Without<Host>,
            Without<Targeting>,
            Without<VirusAttached>,
        ),
    >,
    targets: Query<Entity, (With<Transform>, With<Host>, Without<Hostile>)>,
) {
    let targets_list: Vec<Entity> = targets.iter().collect();
    for virus in viruses.iter_mut() {
        if let Some(&random_target) = targets_list.choose(&mut rand::rng()) {
            commands.entity(virus).insert(Targeting(random_target));
        }
    }
}

fn unset_nonexisting_target(
    mut commands: Commands,
    seekers: Query<(Entity, &Targeting), (With<Hostile>, Without<Host>)>,
    targets: Query<Entity, (With<Host>, Without<Targeting>)>,
) {
    for (virus, targeting) in seekers {
        if !targets.contains(targeting.0) {
            commands.entity(virus).remove::<Targeting>();
        }
    }
}

pub const VIRUS_SPEED: f32 = 20.0;
const FAST_ROTATE_DISTANCE: f32 = 20.0;

fn set_velocity(
    time: Res<Time>,
    mut viruses: Query<(&mut Velocity, &Transform, &Targeting, &TurnSpeed)>,
    targets: Query<&Transform>,
) {
    // go towards the target
    for (mut velocity, seeker_transform, targeting, turn_speed) in viruses.iter_mut() {
        let Ok(target) = targets.get(targeting.0) else {
            continue;
        };

        let to_target = target.translation.xy() - seeker_transform.translation.xy();
        if to_target.length() < 0.01 {
            velocity.value = Vec3::ZERO;
        } else if to_target.length() < FAST_ROTATE_DISTANCE || velocity.value.length() < 0.01 {
            velocity.value = to_target.extend(0.0).normalize() * VIRUS_SPEED;
        } else {
            let new_direction = velocity
                .value
                .xy()
                .rotate_towards(to_target, turn_speed.0 * time.delta_secs());

            velocity.value = new_direction.normalize_or_zero().extend(0.) * VIRUS_SPEED;
        }
    }
    //
}
