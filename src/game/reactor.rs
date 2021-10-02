use super::*;

pub struct Reactor {
    pub circle: Circle,
    pub max_health: f32,
    pub health: f32,
}

impl Reactor {
    pub fn new(circle: Circle, max_health: f32) -> Self {
        Self {
            circle,
            max_health,
            health: max_health,
        }
    }

    /// Damages the reactor and returns whether reactor is still alive or not
    pub fn damage(&mut self, damage: f32) -> bool {
        self.health -= damage;
        let alive = self.health > 0.0;
        if !alive {
            self.health = self.max_health;
        }
        alive
    }
}
