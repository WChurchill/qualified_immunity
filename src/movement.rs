use bevy::prelude::*;

use crate::level::Directional;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_rotation));
    }
}

fn update_position(time: Res<Time>, mut sprite: Query<(&Velocity, &mut Transform)>) {
    for (logo, mut transform) in &mut sprite {
        transform.translation += logo.value * time.delta_secs();
    }
}

fn update_rotation(mut sprite: Query<(&Velocity, &mut Transform), With<Directional>>) {
    for (velocity, mut transform) in &mut sprite {
        let direction = velocity.value.normalize_or_zero();
        if direction == Vec3::ZERO {
            continue;
        }

        let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

        transform.rotation = rotation;
    }
}
