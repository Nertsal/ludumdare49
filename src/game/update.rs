use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        self.control_player(delta_time);

        self.movement(delta_time);
    }

    fn movement(&mut self, delta_time: f32) {
        self.player.rigid_circle.move_delta(delta_time);
    }

    fn control_player(&mut self, delta_time: f32) {
        use geng::Key;

        let window = self.geng.window();

        let mut direction = Vec2::ZERO;
        if window.is_key_pressed(Key::A) {
            direction += vec2(-1.0, 0.0);
        }
        if window.is_key_pressed(Key::D) {
            direction += vec2(1.0, 0.0);
        }
        if window.is_key_pressed(Key::S) {
            direction += vec2(0.0, -1.0);
        }
        if window.is_key_pressed(Key::W) {
            direction += vec2(0.0, 1.0);
        }

        self.player.target_velocity_direction(direction, delta_time);
    }
}
