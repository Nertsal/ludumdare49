use super::*;

#[derive(Clone)]
pub enum Effect {
    HealReactor { heal: f32 },
    ExplodeReactor,
}

impl GameState {
    pub fn apply_effect(&mut self, effect: Effect) {
        match effect {
            Effect::HealReactor { heal } => {
                self.reactor.health += heal;
            }
            Effect::ExplodeReactor => {
                self.explode_reactor();
            }
        }
    }
}
