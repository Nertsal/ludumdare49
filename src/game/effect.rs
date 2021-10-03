use super::*;

pub enum Effect {
    HealReactor { heal: f32 },
}

impl GameState {
    pub fn apply_effect(&mut self, effect: Effect) {
        match effect {
            Effect::HealReactor { heal } => {
                self.reactor.health += heal;
            }
        }
    }
}
