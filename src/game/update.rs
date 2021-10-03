use super::collision::Collision;
use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        self.time_scale(delta_time);
        let delta_time = delta_time * self.time_scale;

        self.scale_difficulty(delta_time);

        self.update_reactor(delta_time);
        self.update_particles(delta_time);

        self.control_player(delta_time);

        self.movement(delta_time);
        self.collide(delta_time);

        self.break_asteroids();
        self.clean();
        self.spawner(delta_time);

        self.spawn_particles();
    }

    fn time_scale(&mut self, delta_time: f32) {
        let target_scale = if self.is_shop_open { 0.0 } else { 1.0 };

        // Interpolate towards target scale
        let mut delta = target_scale - self.time_scale;
        let max_delta = delta_time / TIME_SCALE_CHANGE_TIME;
        if delta.abs() > max_delta {
            delta = delta.signum() * max_delta;
        }
        self.time_scale += delta;
    }

    fn scale_difficulty(&mut self, delta_time: f32) {
        self.difficulty += delta_time * DIFFICULTY_SCALE;
        self.spawn_delay =
            (START_SPAWN_DELAY - self.difficulty * SPAWN_DELAY_SCALE).max(SPAWN_DELAY_MIN);
    }

    fn update_reactor(&mut self, delta_time: f32) {
        self.reactor.circle.color = self.reactor.health_color();
        if self.reactor.health <= 0.0 {
            self.transition_delay -= delta_time;
        }
    }

    fn update_particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.rigid_circle.circle.color.a -= delta_time / PARTICLE_DECAY_TIME;
        }
    }

    fn control_player(&mut self, delta_time: f32) {
        use geng::Key;
        let window = self.geng.window();

        let mut linear = 0.0;
        if window.is_key_pressed(Key::W) || window.is_key_pressed(Key::Up) {
            linear += 1.0;
        }
        if window.is_key_pressed(Key::S) || window.is_key_pressed(Key::Down) {
            linear -= 0.1;
        }

        let mut turn = 0.0;
        if window.is_key_pressed(Key::A) || window.is_key_pressed(Key::Left) {
            turn += 1.0;
        }
        if window.is_key_pressed(Key::D) || window.is_key_pressed(Key::Right) {
            turn -= 1.0;
        }

        self.player.control(linear, turn, delta_time);
    }

    fn movement(&mut self, delta_time: f32) {
        // Move player
        self.player.move_delta(delta_time);

        // Move asteroids
        for asteroid in &mut self.asteroids {
            asteroid.rigid_circle.move_delta(delta_time);
        }

        // Move particles
        for particle in &mut self.particles {
            particle.rigid_circle.move_delta(delta_time);
        }
    }

    fn collide(&mut self, _delta_time: f32) {
        // Player - Border
        if let Some(collision) =
            Collision::circle_border(&self.player.rigid_circle.circle, &self.border)
        {
            collide::collide_rigid_static(&mut self.player.rigid_circle, collision);
        }

        // Player - Reactor
        if let Some(collision) =
            Collision::circle_circle(&self.player.rigid_circle.circle, &self.reactor.circle)
        {
            collide::collide_rigid_static(&mut self.player.rigid_circle, collision);
        }

        // Player - Asteroid
        let player = &mut self.player.rigid_circle;
        for asteroid in &mut self.asteroids {
            if let Some(collision) =
                Collision::circle_circle(&player.circle, &asteroid.rigid_circle.circle)
            {
                collide::collide_rigid_rigid(player, &mut asteroid.rigid_circle, collision);
                asteroid.break_self = true;
            }
        }

        // Reactor - Asteroid
        let mut game_over = false;
        let reactor = &mut self.reactor;
        for asteroid in &mut self.asteroids {
            if let Some(collision) =
                Collision::circle_circle(&asteroid.rigid_circle.circle, &reactor.circle)
            {
                collide::collide_rigid_static(&mut asteroid.rigid_circle, collision);
                asteroid.break_self = true;
                let was_alive = reactor.health > 0.0;
                let alive = reactor.damage(asteroid.rigid_circle.mass);
                if was_alive && !alive {
                    // Game over
                    game_over = true;
                }
            }
        }

        if game_over {
            self.explode_reactor();
        }
    }

    /// Break asteroids
    fn break_asteroids(&mut self) {
        let mut new_asteroids = Vec::new();

        // Prepare new asteroids
        for break_asteroid in self.asteroids.iter().filter(|asteroid| asteroid.break_self) {
            // Score
            self.score += 1;
            self.money += 1;

            // Particles
            self.particle_queue.push(asteroid_particles(break_asteroid));

            // Sound
            self.assets.sounds.asteroid_hit.play();

            // Prepare
            let velocity = break_asteroid.rigid_circle.linear_velocity;
            let len = velocity.len();

            let velocity_norm = if len > 1e-5 { velocity / len } else { velocity };
            let perpendicular = velocity_norm.rotate_90();

            let mut rng = global_rng();
            let variation = rng.gen_range(0.0..=ASTEROID_BREAK_RADIUS_VARIATION);

            let position = break_asteroid.rigid_circle.circle.position;
            let radius = break_asteroid.rigid_circle.circle.radius / 2.0;
            let mass = break_asteroid.rigid_circle.mass / 2.0;

            let new_velocity = (velocity + perpendicular * len) / 2.0;
            let circle = Circle::new(
                position,
                rng.gen_range(0.0..6.0),
                radius - variation,
                Color::WHITE,
            );
            let asteroid = Asteroid::new(circle, mass, new_velocity, rng.gen_range(-1.5..1.5));
            new_asteroids.push(asteroid);

            let new_velocity = (velocity - perpendicular * len) / 2.0;
            let circle = Circle::new(
                position,
                rng.gen_range(0.0..6.0),
                radius + variation,
                Color::WHITE,
            );
            let asteroid = Asteroid::new(circle, mass, new_velocity, rng.gen_range(-1.5..1.5));
            new_asteroids.push(asteroid);
        }

        // Destroy old asteroids
        self.asteroids.retain(|asteroid| !asteroid.break_self);

        // Spawn new asteroids
        for new_asteroid in new_asteroids {
            self.asteroids.push(new_asteroid);
        }
    }

    /// Destroy asteroids and particles that are out of bounds or very small
    fn clean(&mut self) {
        let border = &self.border;

        // Asteroids
        self.asteroids.retain(|asteroid| {
            asteroid.rigid_circle.circle.radius
                > ASTEROID_DESTROY_SIZE + ASTEROID_BREAK_RADIUS_VARIATION
                && border.contains(asteroid.rigid_circle.circle.position)
        });

        // Particles
        self.particles.retain(|particle| {
            particle.rigid_circle.circle.color.a > 0.0
                && border.contains(particle.rigid_circle.circle.position)
        });
    }

    fn spawner(&mut self, delta_time: f32) {
        self.spawn_timer -= delta_time;
        if self.spawn_timer <= 0.0 {
            self.spawn_timer = self.spawn_delay;

            // Spawn a new asteroid
            self.spawn_asteroid();
        }
    }

    pub fn explode_reactor(&mut self) {
        // Sound
        self.assets.sounds.explosion.play();

        // Explode all asteroids
        for asteroid in self.asteroids.drain(..) {
            self.particle_queue.push(asteroid_particles(&asteroid));
        }

        // Reset spawn timer
        self.spawn_timer = self.spawn_delay;
    }

    fn spawn_particles(&mut self) {
        let particles = std::mem::take(&mut self.particle_queue);
        for particle_group in particles {
            self.spawn_particle_group(particle_group);
        }
    }
}

fn asteroid_particles(asteroid: &Asteroid) -> ParticleGroup {
    let radius = asteroid.rigid_circle.circle.radius / 2.5;
    ParticleGroup {
        position: asteroid.rigid_circle.circle.position,
        amount_range: 5..10,
        radius_range: radius - 0.1..radius + 0.1,
        initial_velocity: Vec2::ZERO,
        velocity_offset_x_range: -5.0..5.0,
        velocity_offset_y_range: -5.0..5.0,
        color_reference: asteroid.rigid_circle.circle.color,
        color_alpha: PARTICLE_ALPHA,
    }
}
