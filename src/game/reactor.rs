use super::*;

pub struct Reactor {
    pub circle: Circle,
    pub max_health: f32,
    pub health: f32,
    pub explode_delay: f32,
    pub explode_cooldown: f32,
}

impl Reactor {
    pub fn new(circle: Circle, max_health: f32, explode_delay: f32) -> Self {
        assert!(max_health > 0.0);
        Self {
            circle,
            max_health,
            health: max_health,
            explode_delay,
            explode_cooldown: explode_delay,
        }
    }

    pub fn health_circle(&self) -> Circle {
        Circle {
            position: self.circle.position,
            radius: self.circle.radius * self.health / self.max_health,
            color: REACTOR_HEALTH_COLOR,
        }
    }

    /// Damages the reactor and returns whether reactor is still alive or not
    pub fn damage(&mut self, damage: f32) -> bool {
        self.health -= damage;
        let alive = self.health > 0.0;
        if !alive {
            self.health = 0.0;
        }
        alive
    }
}
