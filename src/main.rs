use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_position, update_velocity))
        .run();
}

const MOVEMENT_SPEED: f32 = 150.;

#[derive(Component)]
struct Velocity {
    pub value: Vec3,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("white_blood_cell.png")),
        Transform::from_xyz(0., 0., 0.),
        Velocity {
            value: Vec3::new(MOVEMENT_SPEED, 0., 0.),
        },
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

fn update_position(time: Res<Time>, mut sprite: Query<(&Velocity, &mut Transform)>) {
    for (logo, mut transform) in &mut sprite {
        transform.translation += logo.value * time.delta_secs();
    }
}
