use bevy::color::palettes::css::*;
use bevy::math::{bounding::*, *};
use bevy::{platform::collections::HashMap, prelude::*};

use crate::enemy::Hostile;
use crate::schedule::InGameSet;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (draw_hitbox, collision_detection).in_set(InGameSet::CollisionDetection),
        );
        app.add_systems(
            Update,
            ((handle_collisions::<Hostile>), despawn_collided)
                .chain()
                .in_set(InGameSet::DespawnEntities),
        )
        .add_event::<CollisionEvent>();
    }
}

#[derive(Debug, Clone)]
pub enum Hitbox {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Component, Debug, Clone)]
pub struct Collider {
    pub hitbox: Hitbox,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn from_hitbox(hitbox: Hitbox) -> Self {
        Self {
            hitbox: hitbox,
            colliding_entities: vec![],
        }
    }
}

fn draw_hitbox(mut gizmos: Gizmos, query: Query<(&Collider, &Transform)>) {
    let color = YELLOW;
    for (collider, transform) in query.iter() {
        match collider.hitbox {
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

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a == entity_b {
                continue;
            }

            let hit: bool = match collider_a.hitbox {
                Hitbox::Circle(circle_a) => match collider_b.hitbox {
                    Hitbox::Circle(circle_b) => false,
                    Hitbox::Rectangle(rect_b) => false,
                },
                Hitbox::Rectangle(rect_a) => match collider_b.hitbox {
                    Hitbox::Circle(circle_b) => false,
                    Hitbox::Rectangle(rect_b) => {
                        let mut aabb = Aabb2d::new(transform_a.translation().xy(), rect_a.size());
                        aabb.rotate_by(Rot2::radians(
                            transform_a.rotation().to_euler(EulerRot::ZYX).0,
                        ));
                        let mut iso_b: Aabb2d =
                            Aabb2d::new(transform_b.translation().xy(), rect_b.half_size);
                        iso_b.rotate_by(Rot2::radians(
                            transform_b.rotation().to_euler(EulerRot::ZYX).0,
                        ));
                        aabb.intersects(&iso_b)
                    }
                },
            };

            if hit {
                colliding_entities
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push(entity_b);
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self {
            entity,
            collided_entity,
        }
    }
}

fn handle_collisions<T: Component>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            collision_event_writer.write(CollisionEvent::new(entity, collided_entity));
        }
    }
}

fn despawn_collided(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionEvent>,
    query: Query<Entity, With<Collider>>,
) {
    for &CollisionEvent {
        entity,
        collided_entity,
    } in collision_event_reader.read()
    {
        let Ok(_attacker) = query.get(collided_entity) else {
            continue;
        };
        let Ok(_victim) = query.get(entity) else {
            continue;
        };

        commands.entity(entity).despawn();
        commands.entity(collided_entity).despawn();
    }
}
