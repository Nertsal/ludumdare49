use super::collision::Collision;
use super::*;

pub fn collide_rigid_static(rigid: &mut RigidCircle, stat: &Circle, collision: Collision) {
    // Move rigid circle
    rigid
        .circle
        .move_delta(collision.penetration * -collision.normal);

    // Change ridid circle's velocity
    let bounciness = 0.0;
    let projection = Vec2::dot(rigid.velocity, collision.normal) * collision.normal;
    rigid.velocity -= projection * (1.0 + bounciness);
}
