use bevy::prelude::*;

use crate::collision::Collider;
use crate::movement::{Directional, Velocity};

// Should only attack once and then die.
#[derive(Component)]
pub struct OneShotAttacker;

#[derive(Bundle)]
pub struct VirusBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub velocity: Velocity,
    pub marker: Directional,
    pub enemy_class: OneShotAttacker,
}
