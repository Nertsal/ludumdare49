use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            "Ludum Dare 49 - Unstable",
            Vec2::ZERO,
            geng::TextAlign::CENTER,
            1.0,
            Color::WHITE,
        );
    }
}
