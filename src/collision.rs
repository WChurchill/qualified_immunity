use bevy::color::palettes::css::*;
use bevy::math::*;
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_hitbox);
    }
}

#[derive(Component, Clone)]
pub enum Hitbox {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn draw_hitbox(mut gizmos: Gizmos, query: Query<(&Hitbox, &Transform)>) {
    let color = YELLOW;
    for (hitbox, transform) in query.iter() {
        match hitbox {
            Hitbox::Circle(c) => {
                gizmos.circle_2d(transform.translation.xy(), c.radius, color);
            }
            Hitbox::Rectangle(r) => {
                gizmos.rect_2d(
                    Isometry2d::new(
                        transform.translation.xy(),
                        Rot2::radians(transform.rotation.to_euler(EulerRot::ZYX).0),
                    ),
                    r.size(),
                    color,
                );
            }
        };
    }
}
