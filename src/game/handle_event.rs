use super::*;

impl GameState {
    pub fn hande_event_impl(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown {
                key: geng::Key::Space,
            } => {
                self.explode_reactor();
            }
            _ => (),
        }
    }
}
