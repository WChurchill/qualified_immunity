mod movement;
mod player;

use bevy::prelude::*;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
