use super::*;

pub struct Player {
    pub rigid_circle: RigidCircle,

    /// Rotation starting from up, counter clockwise
    pub max_rotational_speed: f32,
    pub rotational_acceleration: f32,

    pub max_linear_speed: f32,
    pub linear_acceleration: f32,
    pub is_accelerating: bool,
}

impl Player {
    pub fn new(
        rigid_circle: RigidCircle,
        max_linear_speed: f32,
        linear_acceleration: f32,
        max_rotational_speed: f32,
        rotational_acceleration: f32,
    ) -> Self {
        Self {
            rigid_circle,

            max_rotational_speed,
            rotational_acceleration,

            max_linear_speed,
            linear_acceleration,
            is_accelerating: false,
        }
    }

    /// Returns direction forward
    pub fn forward(&self) -> Vec2<f32> {
        let angle = self.rigid_circle.circle.rotation + std::f32::consts::FRAC_PI_2;
        let (sin, cos) = angle.sin_cos();
        vec2(cos, sin)
    }

    pub fn move_delta(&mut self, delta_time: f32) {
        self.rigid_circle.move_delta(delta_time);
    }

    pub fn control(&mut self, target_linear: f32, target_rotational: f32, delta_time: f32) {
        // Linear
        assert!(target_linear >= -0.1);
        assert!(target_linear <= 1.0);
        let target_linear_speed = self.max_linear_speed * target_linear;
        self.is_accelerating = target_linear_speed > 0.0;

        let forward = self.forward();
        let target_velocity = forward * target_linear_speed;
        let mut delta = target_velocity - self.rigid_circle.linear_velocity;
        let max_delta = self.linear_acceleration * delta_time;
        let len = delta.len();
        if len > max_delta {
            delta = delta / len * max_delta;
        }

        self.rigid_circle.linear_velocity += delta;

        // Rotational
        assert!(target_rotational >= -1.0);
        assert!(target_rotational <= 1.0);
        let target_rotational_speed = self.max_rotational_speed * target_rotational;

        let mut delta = target_rotational_speed - self.rigid_circle.rotation_velocity;
        let max_delta = self.rotational_acceleration * delta_time;
        if delta.abs() > max_delta {
            delta = delta.signum() * max_delta;
        }

        self.rigid_circle.rotation_velocity += delta;
    }
}
