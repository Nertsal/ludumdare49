use super::*;

impl GameState {
    pub fn spawn_asteroid(&mut self) {
        let (position, _) = self.gen_border_pos();
        debug_assert_ne!(position, Vec2::ZERO);
        let direction = (Vec2::ZERO - position).normalize();

        let mut rng = global_rng();
        let random = rng.gen();
        let mass = fraction_to_range(random, ASTEROID_MASS_MIN, ASTEROID_MASS_MAX);
        let radius = fraction_to_range(random, ASTEROID_RADIUS_MIN, ASTEROID_RADIUS_MAX);
        let speed = rng.gen_range(ASTEROID_SPEED_MIN..=ASTEROID_SPEED_MAX);

        let circle = Circle::new(position, radius, ASTEROID_COLOR);
        let asteroid = Asteroid::new(circle, mass, direction * speed);

        self.asteroids.push(asteroid);
    }

    /// Returns a postion on the border and a normal pointing inside
    fn gen_border_pos(&self) -> (Vec2<f32>, Vec2<f32>) {
        let mut rng = global_rng();
        if rng.gen_bool(0.5) {
            // Up or Down
            let x = rng.gen_range(self.border.x_min..=self.border.x_max);
            let (y, ny) = if rng.gen_bool(0.5) {
                (self.border.y_min, 1.0)
            } else {
                (self.border.y_max, -1.0)
            };
            (vec2(x, y), vec2(0.0, ny))
        } else {
            // Left or Right
            let y = rng.gen_range(self.border.y_min..=self.border.y_max);
            let (x, nx) = if rng.gen_bool(0.5) {
                (self.border.x_min, 1.0)
            } else {
                (self.border.x_max, -1.0)
            };
            (vec2(x, y), vec2(nx, 0.0))
        }
    }
}

fn fraction_to_range(fraction: f32, range_min: f32, range_max: f32) -> f32 {
    range_min + (range_max - range_min) * fraction
}
