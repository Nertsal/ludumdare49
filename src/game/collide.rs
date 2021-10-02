use super::collision::Collision;
use super::*;

/// Performs collision between a rigid and a static circles. Ensure that when cheking for collision,
/// the first argument was the rigid circle, and the second argument was the static circle.
pub fn collide_rigid_static(rigid: &mut RigidCircle, collision: Collision) {
    // Move rigid circle
    rigid
        .circle
        .move_delta(collision.penetration * -collision.normal);

    // Change ridid circle's velocity
    let bounciness = 0.0;
    let projection = Vec2::dot(rigid.velocity, collision.normal) * collision.normal;
    rigid.velocity -= projection * (1.0 + bounciness);
}

pub fn collide_rigid_rigid(rigid: &mut RigidCircle, other: &mut RigidCircle, collision: Collision) {
    // Move rigid circles
    let delta = collision.penetration * collision.normal / 2.0;
    rigid.circle.move_delta(-delta);
    other.circle.move_delta(delta);

    // Change velocities
    let projection = Vec2::dot(rigid.velocity, collision.normal) * collision.normal;
    let other_projection = Vec2::dot(other.velocity, collision.normal) * collision.normal;
    rigid.velocity += other_projection;
    other.velocity += projection;
}
