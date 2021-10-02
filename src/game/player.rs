use super::*;

pub struct Player {
    pub rigid_circle: RigidCircle,
    pub max_speed: f32,
    pub acceleration: f32,
    pub is_accelerating: bool,
}

impl Player {
    pub fn new(rigid_circle: RigidCircle, max_speed: f32, acceleration: f32) -> Self {
        Self {
            rigid_circle,
            max_speed,
            acceleration,
            is_accelerating: false,
        }
    }

    pub fn target_velocity_direction(&mut self, direction: Vec2<f32>, delta_time: f32) {
        // Clamp length
        let len = direction.len();
        let target_direction = if len > 1.0 {
            direction / len
        } else {
            direction
        };

        self.is_accelerating = len > 0.0;

        let target_speed = target_direction * self.max_speed;
        self.rigid_circle
            .target_velocity(target_speed, self.acceleration, delta_time);
    }
}
