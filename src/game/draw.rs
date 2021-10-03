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

        // Draw reactor
        self.draw_textured_circle(
            framebuffer,
            &self.reactor.circle,
            Some(&self.assets.nuclear),
        );

        // Reactor health
        self.assets.font.draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            "Reactor Stability",
            vec2(10.0, framebuffer_size.y - 50.0),
            geng::TextAlign::LEFT,
            30.0,
            Color::WHITE,
        );

        // Draw reactor health
        let bar_position = vec2(10.0, framebuffer_size.y - 80.0);
        let bar_width = 175.0;
        let bar_height = 20.0;
        let bar_aabb = AABB::point(bar_position).extend_positive(vec2(bar_width, bar_height));
        self.geng.draw_2d().quad(
            framebuffer,
            &geng::PixelPerfectCamera,
            bar_aabb,
            Color::rgb(0.0, 0.3, 0.0),
        );
        let offset = 2.0;
        let health_aabb = bar_aabb.extend_uniform(-offset).extend_positive(vec2(
            (self.reactor.health / self.reactor.max_health - 1.0) * (bar_width - offset),
            0.0,
        ));
        self.geng.draw_2d().quad(
            framebuffer,
            &geng::PixelPerfectCamera,
            health_aabb,
            Color::rgb(0.0, 0.7, 0.0),
        );

        // Score
        self.assets.font.draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            &format!("SCORE: {}", self.score),
            vec2(framebuffer_size.x - 200.0, framebuffer_size.y - 50.0),
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
