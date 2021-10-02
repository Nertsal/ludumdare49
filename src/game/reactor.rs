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

    pub fn health_color(&self) -> Color<f32> {
        let s = REACTOR_HEALTH_COLOR;
        let e = REACTOR_COLOR;
        let f = 1.0 - self.health / self.max_health;
        Color::rgb(
            s.r + (e.r - s.r) * f,
            s.g + (e.g - s.g) * f,
            s.b + (e.b - s.b) * f,
        )
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
