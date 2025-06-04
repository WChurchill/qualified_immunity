use avian2d::prelude::*;
use bevy::prelude::*;

use crate::schedule::InGameSet;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_collided.in_set(InGameSet::DespawnEntities));
    }
}

// fn draw_hitbox(mut gizmos: Gizmos, query: Query<(&Collider, &Transform)>) {
//     let color = YELLOW;
//     for (collider, transform) in query.iter() {
//         match collider.hitbox {
//             Hitbox::Circle(c) => {
//                 gizmos.circle_2d(transform.translation.xy(), c.radius, color);
//             }
//             Hitbox::Rectangle(r) => {
//                 gizmos.rect_2d(
//                     Isometry2d::new(
//                         transform.translation.xy(),
//                         Rot2::radians(transform.rotation.to_euler(EulerRot::ZYX).0),
//                     ),
//                     r.size(),
//                     color,
//                 );
//             }
//         };
//     }
// }

fn despawn_collided(mut commands: Commands, query: Query<(Entity, &CollidingEntities)>) {
    for (entity, colliding_entities) in &query {
        if !colliding_entities.is_empty() {
            commands.entity(entity).despawn();
        }
    }
}
