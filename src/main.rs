mod level;
mod movement;
mod player;

use bevy::prelude::*;
use level::LevelPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(LevelPlugin)
        .run();
}
