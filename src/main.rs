mod enemy;
mod host;
mod hud;
mod level;
mod movement;
mod player;
mod player_attack;
mod schedule;

use avian2d::prelude::*;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use host::HostPlugin;
use hud::HUDPlugin;
use level::LevelPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use player_attack::PlayerAttackPlugin;
use schedule::SchedulePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default()) // DO NOT RELEASE
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PlayerAttackPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(HostPlugin)
        .add_plugins(HUDPlugin)
        .run();
}
