use bevy::prelude::*;

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
        app.add_systems(Update, update_position);
    }
}

fn update_position(time: Res<Time>, mut sprite: Query<(&Velocity, &mut Transform)>) {
    for (logo, mut transform) in &mut sprite {
        transform.translation += logo.value * time.delta_secs();
    }
}
