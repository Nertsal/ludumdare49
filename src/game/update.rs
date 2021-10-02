use super::collision::Collision;
use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        self.control_player(delta_time);

        self.movement(delta_time);
        self.collide(delta_time);

        self.spawner(delta_time);
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

    fn movement(&mut self, delta_time: f32) {
        // Move player
        self.player.rigid_circle.move_delta(delta_time);

        // Move asteroids
        for asteroid in &mut self.asteroids {
            asteroid.rigid_circle.move_delta(delta_time);
        }
    }

    fn collide(&mut self, delta_time: f32) {
        // Player - Reactor
        if let Some(collision) =
            Collision::circle_circle(&self.player.rigid_circle.circle, &self.reactor.circle)
        {
            collide::collide_rigid_static(&mut self.player.rigid_circle, collision);
        }

        // Player - Asteroid

        // Reactor - Asteroid
    }

    fn spawner(&mut self, delta_time: f32) {
        self.spawn_timer -= delta_time;
        if self.spawn_timer <= 0.0 {
            self.spawn_timer = self.spawn_delay;

            // Spawn a new asteroid
            self.spawn_asteroid();
        }
    }
}
