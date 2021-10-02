use super::*;

pub struct Circle {
    pub position: Vec2<f32>,
    pub radius: f32,
    pub color: Color<f32>,
}

impl Circle {
    pub fn new(position: Vec2<f32>, radius: f32, color: Color<f32>) -> Self {
        Self {
            position,
            radius,
            color,
        }
    }

    pub fn move_delta(&mut self, delta: Vec2<f32>) {
        self.position += delta;
    }

    /// Returns the AABB around itself
    pub fn aabb(&self) -> AABB<f32> {
        AABB::point(self.position).extend_uniform(self.radius)
    }
}

pub struct RigidCircle {
    pub circle: Circle,
    pub velocity: Vec2<f32>,
    pub mass: f32,
}

impl RigidCircle {
    pub fn new(circle: Circle, mass: f32) -> Self {
        Self {
            circle,
            mass,
            velocity: Vec2::ZERO,
        }
    }

    pub fn move_delta(&mut self, delta_time: f32) {
        self.circle.move_delta(self.velocity * delta_time);
    }

    /// Accelerate towards target velocity
    pub fn target_velocity(
        &mut self,
        target_velocity: Vec2<f32>,
        acceleration: f32,
        delta_time: f32,
    ) {
        let max_accel = acceleration * delta_time;

        let mut delta = target_velocity - self.velocity;
        let delta_len = delta.len();
        if delta_len > max_accel {
            delta = delta / delta_len * max_accel;
        }

        self.accelerate(delta);
    }

    pub fn accelerate(&mut self, acceleration: Vec2<f32>) {
        self.velocity += acceleration;
    }
}
