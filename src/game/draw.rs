use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw player
        self.draw_circle(framebuffer, &self.player.rigid_circle.circle);

        // Draw reactor
        self.draw_circle(framebuffer, &self.reactor.circle);
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
