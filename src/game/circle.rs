use super::*;

pub struct Circle {
    pub position: Vec2<f32>,
    pub rotation: f32,
    pub radius: f32,
    pub color: Color<f32>,
}

impl Circle {
    pub fn new(position: Vec2<f32>, rotation: f32, radius: f32, color: Color<f32>) -> Self {
        Self {
            position,
            rotation,
            radius,
            color,
        }
    }

    pub fn move_delta(&mut self, delta: Vec2<f32>) {
        self.position += delta;
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.rotation += rotation;

        // Clamp rotation 0..2 * PI
        let period = std::f32::consts::PI * 2.0;
        while self.rotation < 0.0 {
            self.rotation += period;
        }
        while self.rotation >= period {
            self.rotation -= period
        }
    }

    /// Returns the AABB around itself
    pub fn aabb(&self) -> AABB<f32> {
        AABB::point(self.position).extend_uniform(self.radius)
    }
}

pub struct RigidCircle {
    pub circle: Circle,
    pub linear_velocity: Vec2<f32>,
    pub rotation_velocity: f32,
    pub mass: f32,
}

impl RigidCircle {
    pub fn new(circle: Circle, mass: f32) -> Self {
        Self {
            circle,
            mass,
            linear_velocity: Vec2::ZERO,
            rotation_velocity: 0.0,
        }
    }

    pub fn move_delta(&mut self, delta_time: f32) {
        self.circle.move_delta(self.linear_velocity * delta_time);
        self.circle.rotate(self.rotation_velocity * delta_time);
    }
}
