use super::*;

#[derive(Debug)]
pub struct Collision {
    pub penetration: f32,
    /// Normal of the collision pointing from other to one
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
        let penetration = radii - distance;
        if penetration <= 0.0 {
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

    /// Checks for collisions inside the border.
    /// Returns None if there is no collision, otherwise returns Some(collision).
    pub fn circle_border(circle: &Circle, border: &AABB<f32>) -> Option<Self> {
        let circle_aabb = AABB::point(circle.position).extend_uniform(circle.radius);

        debug_assert!(circle_aabb.width() < border.width());
        debug_assert!(circle_aabb.height() < border.height());

        // Check left side
        let penetration = border.x_min - circle_aabb.x_min;
        if penetration > 0.0 {
            return Some(Self {
                penetration,
                normal: vec2(1.0, 0.0),
            });
        }

        // Check right side
        let penetration = circle_aabb.x_max - border.x_max;
        if penetration > 0.0 {
            return Some(Self {
                penetration: penetration,
                normal: vec2(-1.0, 0.0),
            });
        }

        // Check bottom side
        let penetration = border.y_min - circle_aabb.y_min;
        if penetration > 0.0 {
            return Some(Self {
                penetration: penetration,
                normal: vec2(0.0, 1.0),
            });
        }

        // Check top side
        let penetration = circle_aabb.y_max - border.y_max;
        if penetration > 0.0 {
            return Some(Self {
                penetration: penetration,
                normal: vec2(0.0, -1.0),
            });
        }

        None
    }
}
