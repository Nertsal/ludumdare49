use super::collision::Collision;
use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
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

    fn scale_difficulty(&mut self, delta_time: f32) {
        self.difficulty += delta_time * DIFFICULTY_SCALE;
        self.spawn_delay =
            (START_SPAWN_DELAY - self.difficulty * SPAWN_DELAY_SCALE).max(SPAWN_DELAY_MIN);
    }

    fn update_reactor(&mut self, delta_time: f32) {
        if self.reactor.explode_cooldown > 0.0 {
            self.reactor.explode_cooldown -= delta_time;
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
        let reactor = &mut self.reactor;
        for asteroid in &mut self.asteroids {
            if let Some(collision) =
                Collision::circle_circle(&asteroid.rigid_circle.circle, &reactor.circle)
            {
                collide::collide_rigid_static(&mut asteroid.rigid_circle, collision);
                asteroid.break_self = true;
                reactor.damage(asteroid.rigid_circle.mass);
            }
        }
    }

    /// Break asteroids
    fn break_asteroids(&mut self) {
        let mut new_asteroids = Vec::new();

        // Prepare new asteroids
        for break_asteroid in self.asteroids.iter().filter(|asteroid| asteroid.break_self) {
            // Particles
            self.particle_queue.push(asteroid_particles(break_asteroid));

            // Prepare
            let velocity = break_asteroid.rigid_circle.velocity;
            let len = velocity.len();

            let velocity_norm = if len > 1e-5 { velocity / len } else { velocity };
            let perpendicular = velocity_norm.rotate_90();

            let mut rng = global_rng();
            let variation = rng.gen_range(0.0..=ASTEROID_BREAK_RADIUS_VARIATION);

            let position = break_asteroid.rigid_circle.circle.position;
            let radius = break_asteroid.rigid_circle.circle.radius / 2.0;
            let mass = break_asteroid.rigid_circle.mass / 2.0;

            let new_velocity = (velocity + perpendicular * len) / 2.0;
            let circle = Circle::new(position, radius - variation, ASTEROID_COLOR);
            let asteroid = Asteroid::new(circle, mass, new_velocity);
            new_asteroids.push(asteroid);

            let new_velocity = (velocity - perpendicular * len) / 2.0;
            let circle = Circle::new(position, radius + variation, ASTEROID_COLOR);
            let asteroid = Asteroid::new(circle, mass, new_velocity);
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
        if self.reactor.explode_cooldown <= 0.0 {
            // Do something with the reactor
            self.reactor.explode_cooldown = self.reactor.explode_delay;

            // Explode all asteroids
            for asteroid in self.asteroids.drain(..) {
                self.particle_queue.push(asteroid_particles(&asteroid));
            }
        }
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
