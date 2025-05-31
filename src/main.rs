use bevy::prelude::*;

fn hello_world() {
    println!("Hello, bevy!");
}

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
