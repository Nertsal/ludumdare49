use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw border
        self.geng
            .draw_2d()
            .quad(framebuffer, &self.camera, self.border, Color::WHITE);

        // Draw player
        self.draw_circle(framebuffer, &self.player.rigid_circle.circle);

        // Draw reactor
        self.draw_circle(framebuffer, &self.reactor.circle);
        self.draw_circle(framebuffer, &self.reactor.health_circle());

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
