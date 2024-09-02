use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Return a tuple of two `Entity`s if they started colliding in this `CollisionEvent`, or `None`
/// otherwise.
///
/// `query1` should be a `Query<&T>` where `T` is the component type of the entity you want to
/// appear in the `.0` position of the return tuple. `query2` is the same but for the `.1` position
/// of the return tuple.
pub fn collision_started<T, U>(
    query1: &Query<&T>,
    query2: &Query<&U>,
    collision_event: &CollisionEvent,
) -> Option<(Entity, Entity)>
where
    T: Component,
    U: Component,
{
    if let CollisionEvent::Started(entity_a, entity_b, _) = collision_event {
        if query1.get(*entity_a).is_ok() && query2.get(*entity_b).is_ok() {
            return Some((*entity_a, *entity_b));
        } else if query1.get(*entity_b).is_ok() && query2.get(*entity_a).is_ok() {
            return Some((*entity_b, *entity_a));
        }
    }
    None
}

/// Return a tuple of two `Entity`s if they stopped colliding in this `CollisionEvent`, or `None`
/// otherwise.
///
/// `query1` should be a `Query<&T>` where `T` is the component type of the entity you want to
/// appear in the `.0` position of the return tuple. `query2` is the same but for the `.1` position
/// of the return tuple.
pub fn collision_stopped<T, U>(
    query1: &Query<&T>,
    query2: &Query<&U>,
    collision_event: &CollisionEvent,
) -> Option<(Entity, Entity)>
where
    T: Component,
    U: Component,
{
    if let CollisionEvent::Stopped(entity_a, entity_b, _) = collision_event {
        if query1.get(*entity_a).is_ok() && query2.get(*entity_b).is_ok() {
            return Some((*entity_a, *entity_b));
        } else if query1.get(*entity_b).is_ok() && query2.get(*entity_a).is_ok() {
            return Some((*entity_b, *entity_a));
        }
    }
    None
}

/// A function that spawns a compound collider with three rectangles bordering the
/// left, right, and bottom of the default level, perfect for a simple game with gravity.
pub fn create_gravity2d_boundaries(commands: &mut Commands) {
    commands.spawn((
        Name::new("Physics Boundaries"),
        RigidBody::Fixed,
        TransformBundle::default(),
        Friction::new(0.0),
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Collider::compound(vec![
            (Vec2::new(0.0, -360.0), 0.0, Collider::cuboid(640.0, 10.0)),
            (Vec2::new(-640.0, 0.0), 0.0, Collider::cuboid(10.0, 360.0)),
            (Vec2::new(640.0, 0.0), 0.0, Collider::cuboid(10.0, 360.0)),
        ]),
    ));
}
