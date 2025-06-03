mod collision;
mod level;
mod movement;
mod player;

use bevy::prelude::*;
use collision::CollisionPlugin;
use level::LevelPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}
