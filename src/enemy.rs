use bevy::color::palettes::css::*;
use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::collision::Collider;
use crate::level::Host;
use crate::movement::{Directional, Velocity};
use crate::player::Player;
use crate::schedule::InGameSet;

// Should only attack once and then die.
#[derive(Component)]
pub enum Hostile {
    InfectThenDie,
}

#[derive(Component)]
pub struct Targeting(Entity);

#[derive(Bundle)]
pub struct VirusBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub velocity: Velocity,
    pub marker: Directional,
    pub enemy_class: Hostile,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (set_target, set_velocity).in_set(InGameSet::EntityUpdates),
        );
    }
}

fn set_target(
    mut commands: Commands,
    mut viruses: Query<
        Entity,
        (
            With<Velocity>,
            With<Transform>,
            With<Directional>,
            Without<Targeting>,
        ),
    >,
    targets: Query<Entity, (With<Transform>, With<Host>)>,
) {
    let targets_list: Vec<Entity> = targets.iter().collect();
    for virus in viruses.iter_mut() {
        if let Some(&random_target) = targets_list.choose(&mut rand::rng()) {
            commands.entity(virus).insert(Targeting(random_target));
        }
    }
}

const TARGET_DEBUG_COLOR: Srgba = BLUE;

pub const VIRUS_SPEED: f32 = 20.0;

fn set_velocity(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut viruses: Query<(&mut Velocity, &Transform, &Targeting), With<Directional>>,
    targets: Query<&Transform>,
) {
    // go towards the target
    for (mut velocity, seeker_transform, targeting) in viruses.iter_mut() {
        let Ok(target) = targets.get(targeting.0) else {
            continue;
        };

        gizmos.line_2d(
            seeker_transform.translation.xy(),
            target.translation.xy(),
            TARGET_DEBUG_COLOR,
        );

        let to_target = (target.translation.xy() - seeker_transform.translation.xy()).normalize();
        velocity.value = to_target.extend(0.0) * VIRUS_SPEED;
    }
    //
}
