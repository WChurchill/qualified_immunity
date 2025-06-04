mod collision;
mod enemy;
mod level;
mod movement;
mod player;
mod schedule;

use avian2d::prelude::*;
use bevy::prelude::*;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use level::LevelPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(EnemyPlugin)
        .run();
}
