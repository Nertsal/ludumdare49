use super::*;

pub struct Collision {
    pub penetration: f32,
    /// Normal of the collision pointing from one to other
    pub normal: Vec2<f32>,
}

impl Collision {
    /// Returns None if there is no collision, otherwise returns Some(collision)
    pub fn circle_circle(circle: &Circle, other: &Circle) -> Option<Self> {
        let delta = circle.position - other.position;
        let distance = delta.len();

        if distance == 0.0 {
            // Undefined collision
            return None;
        }

        let radii = circle.radius + other.radius;
        let penetration = distance - radii;
        if penetration > 0.0 {
            // No collision
            return None;
        }

        // Yes collision
        let collision = Self {
            penetration,
            normal: delta.normalize(),
        };
        Some(collision)
    }
}
