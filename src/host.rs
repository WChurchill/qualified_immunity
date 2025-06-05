use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

pub struct HostPlugin;

use crate::enemy::create_virus;
use crate::movement::{Directional, Velocity};
use crate::schedule::InGameSet;

impl Plugin for HostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (decay_cell).in_set(InGameSet::EntityUpdates));
        app.add_systems(Update, (cells_die).in_set(InGameSet::DespawnEntities));
    }
}

#[derive(Component, Clone)]
pub struct Host {
    pub seconds_to_death: f32,
    pub decay_multiplier: f32,
    pub num_offspring: i32,
}

impl Host {
    pub fn new(seconds_to_death: f32, num_offspring: i32) -> Self {
        Host {
            seconds_to_death: seconds_to_death,
            decay_multiplier: 0.0,
            num_offspring: num_offspring,
        }
    }
}

fn decay_cell(time: Res<Time>, mut query: Query<&mut Host>) {
    for mut host in query.iter_mut() {
        host.seconds_to_death -= host.decay_multiplier * time.delta_secs();
    }
}

// TODO: make this system events based.
fn cells_die(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Host, &Transform)>,
) {
    for (entity, host, transform) in query.iter() {
        if host.seconds_to_death > 0.0 {
            continue;
        }

        let mut rng = rand::rng();

        println!("cell ded");
        for _ in 0..host.num_offspring {
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
