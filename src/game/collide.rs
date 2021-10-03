use super::collision::Collision;
use super::*;

/// Performs collision between a rigid and a static circles. Ensure that when cheking for collision,
/// the first argument was the rigid circle, and the second argument was the static circle.
pub fn collide_rigid_static(rigid: &mut RigidCircle, collision: Collision) {
    // Move rigid circle
    rigid
        .circle
        .move_delta(collision.penetration * collision.normal);

    // Change ridid circle's velocity
    let bounciness = 0.0;
    let projection = Vec2::dot(rigid.linear_velocity, collision.normal) * collision.normal;
    rigid.linear_velocity -= projection * (1.0 + bounciness);
}

/// Performs collision between a rigid and a static circles. Ensure that when cheking for collision,
/// the order of the arguments (rigid circles) is the same.
pub fn collide_rigid_rigid(rigid: &mut RigidCircle, other: &mut RigidCircle, collision: Collision) {
    // Move rigid circles
    let delta = collision.penetration * collision.normal / 2.0;
    rigid.circle.move_delta(delta);
    other.circle.move_delta(-delta);

    // Change velocities
    let relative_velocity = other.linear_velocity - rigid.linear_velocity;
    let projection = Vec2::dot(relative_velocity, collision.normal) * collision.normal;
    let total_mass = rigid.mass + other.mass;
    rigid.linear_velocity += projection * other.mass / total_mass;
    other.linear_velocity -= projection * rigid.mass / total_mass;
}
