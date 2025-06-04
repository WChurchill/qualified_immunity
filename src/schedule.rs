use bevy::ecs::schedule::ApplyDeferred;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
                InGameSet::DespawnEntities,
            )
                .chain(),
        )
        .add_systems(Update, ApplyDeferred.before(InGameSet::EntityUpdates));
    }
}
