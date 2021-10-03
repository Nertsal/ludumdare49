use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.framebuffer_size = framebuffer_size;
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw player
        // Rocket
        self.draw_textured_circle(
            framebuffer,
            &self.player.rigid_circle.circle,
            Some(&self.assets.rocket),
        );
        // Rocket booster
        if self.player.is_accelerating {
            self.draw_textured_circle(
                framebuffer,
                &self.player.rigid_circle.circle,
                Some(&self.assets.rocket_booster),
            );
        }

        // Draw reactor
        self.draw_textured_circle(
            framebuffer,
            &self.reactor.circle,
            Some(&self.assets.nuclear),
        );

        // Draw asteroids
        for asteroid in &self.asteroids {
            self.draw_textured_circle(
                framebuffer,
                &asteroid.rigid_circle.circle,
                Some(&self.assets.asteroid),
            );
        }

        // Draw particles
        for particle in &self.particles {
            self.draw_textured_circle(
                framebuffer,
                &particle.rigid_circle.circle,
                Some(&self.assets.asteroid),
            );
        }

        // Score
        self.assets.font.draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            &format!("SCORE: {}", self.score),
            vec2(20.0, framebuffer_size.y - 50.0),
            geng::TextAlign::LEFT,
            40.0,
            Color::WHITE,
        )
    }

    fn draw_textured_circle(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        circle: &Circle,
        texture: Option<&ugli::Texture>,
    ) {
        match texture {
            Some(texture) => {
                let matrix = Mat3::translate(circle.position)
                    * Mat3::rotate(circle.rotation)
                    * Mat3::scale_uniform(circle.radius * 2.0)
                    * Mat3::translate(vec2(-0.5, -0.5));
                self.renderer
                    .draw(framebuffer, &self.camera, matrix, texture, circle.color);
            }
            None => {
                self.geng.draw_2d().circle(
                    framebuffer,
                    &self.camera,
                    circle.position,
                    circle.radius,
                    circle.color,
                );
            }
        }
    }
}
