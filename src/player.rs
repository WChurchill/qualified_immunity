use bevy::prelude::*;

use crate::movement::Velocity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, update_velocity);
    }
}

const MOVEMENT_SPEED: f32 = 150.;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("white_blood_cell.png")),
        Transform::from_xyz(0., 0., 0.),
        Velocity::new(Vec3::new(MOVEMENT_SPEED, 0., 0.)),
    ));
}

fn update_velocity(mut sprite_position: Query<(&mut Velocity, &mut Transform)>) {
    for (mut logo, transform) in &mut sprite_position {
        if transform.translation.x > 200. {
            *logo = Velocity {
                value: Vec3::new(-MOVEMENT_SPEED, 0., 0.),
            };
        } else if transform.translation.x < -200. {
            *logo = Velocity {
                value: Vec3::new(MOVEMENT_SPEED, 0., 0.),
            };
        }
    }
}
