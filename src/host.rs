use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

pub struct HostPlugin;

use crate::enemy::{create_virus, Hostile, Targeting, VirusAttached};
use crate::movement::Velocity;
use crate::schedule::InGameSet;

impl Plugin for HostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (decay_cell).in_set(InGameSet::EntityUpdates));
        app.add_systems(Update, (cells_die).in_set(InGameSet::DespawnEntities));
    }
}

#[derive(Component, Clone)]
pub struct Host;

#[derive(Component, Clone)]
pub struct Infected {
    seconds_to_death: f32,
    decay_multiplier: f32,
    num_offspring: i32,
}

impl Default for Infected {
    fn default() -> Self {
        Infected {
            seconds_to_death: 2.0,
            decay_multiplier: 1.0,
            num_offspring: 4,
        }
    }
}

pub fn handle_infection(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut enemies: Query<(&Hostile, &mut Velocity, &mut Transform), Without<VirusAttached>>,
    host: Query<(&Transform, Option<&Infected>), Without<Hostile>>,
) {
    let Ok((hostile, mut velocity, mut transform)) = enemies.get_mut(trigger.collider) else {
        return;
    };

    if let Hostile::InfectThenDie = hostile {
        velocity.value = Vec3::ZERO;
        commands.entity(trigger.collider).insert(VirusAttached);
        commands.entity(trigger.collider).remove::<Targeting>();

        if let Ok((parent_transform, infected)) = host.get(trigger.target()) {
            transform.translation -= parent_transform.translation;

            match infected {
                Some(_) => {}
                _ => {
                    commands
                        .entity(trigger.target())
                        .insert(Infected::default());
                }
            };
        };

        commands
            .entity(trigger.target())
            .add_child(trigger.collider);
    };
}

fn decay_cell(time: Res<Time>, mut query: Query<&mut Infected>) {
    for mut i in query.iter_mut() {
        i.seconds_to_death -= i.decay_multiplier * time.delta_secs();
    }
}

// TODO: make this system events based.
fn cells_die(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Infected, &Transform)>,
) {
    for (entity, infected, transform) in query.iter() {
        if infected.seconds_to_death > 0.0 {
            continue;
        }

        let mut rng = rand::rng();

        for _ in 0..infected.num_offspring {
            let random_direction = Vec2::from_angle(rng.random_range(0.0..2.0 * PI));
            commands.spawn(create_virus(
                &asset_server,
                random_direction,
                transform.translation.xy(),
            ));
        }
        commands.entity(entity).despawn();
    }
}
