use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw player
        self.draw_circle(framebuffer, &self.player.rigid_circle.circle);

        // Draw reactor
        self.geng.draw_2d().textured_quad(
            framebuffer,
            &self.camera,
            self.reactor.circle.aabb(),
            &self.assets.nuclear,
            self.reactor.health_color(),
        );

        // Draw asteroids
        for asteroid in &self.asteroids {
            self.draw_circle(framebuffer, &asteroid.rigid_circle.circle);
        }

        // Draw particles
        for particle in &self.particles {
            self.draw_circle(framebuffer, &particle.rigid_circle.circle);
        }
    }

    fn draw_circle(&self, framebuffer: &mut ugli::Framebuffer, circle: &Circle) {
        self.geng.draw_2d().circle(
            framebuffer,
            &self.camera,
            circle.position,
            circle.radius,
            circle.color,
        );
    }
}
