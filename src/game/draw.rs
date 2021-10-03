use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw player

        // Rocket
        let circle = &self.player.rigid_circle.circle;
        let matrix = Mat3::translate(circle.position)
            * Mat3::rotate(self.player.rotation)
            * Mat3::scale_uniform(circle.radius * 2.0)
            * Mat3::translate(vec2(-0.5, -0.5));
        self.renderer.draw(
            framebuffer,
            &self.camera,
            matrix,
            &self.assets.rocket,
            Color::WHITE,
        );
        // Rocket booster
        if self.player.is_accelerating {
            self.renderer.draw(
                framebuffer,
                &self.camera,
                matrix,
                &self.assets.rocket_booster,
                Color::WHITE,
            );
        }

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
