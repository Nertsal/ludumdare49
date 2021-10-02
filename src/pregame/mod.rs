use geng::prelude::*;

use crate::Assets;

pub struct PregameState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: geng::Camera2d,
}

impl PregameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 50.0,
            },
        }
    }
}

impl geng::State for PregameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let center = framebuffer_size / 2.0;

        let reactor_pos = vec2(15.0, -5.0);

        // Get distance to reactor
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let mouse_world_pos = self.camera.screen_to_world(framebuffer_size, mouse_pos);
        let distance = (reactor_pos - mouse_world_pos).len();

        let s = Color::rgb(0.0, 0.5, 0.0);
        let e = Color::rgb(0.0, 1.0, 0.0);
        let f = 1.0 - (distance / 40.0).clamp(0.0, 1.0);
        let reactor_color = Color::rgb(
            s.r + (e.r - s.r) * f,
            s.g + (e.g - s.g) * f,
            s.b + (e.b - s.b) * f,
        );

        // Ludum Dare 49 - Unstable
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            "Ludum Dare 49 - Unstable",
            vec2(center.x, 5.0),
            geng::TextAlign::CENTER,
            24.0,
            Color::WHITE,
        );

        // Unstable Asteroids
        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            "Unstable Asteroids",
            vec2(0.0, 15.0),
            geng::TextAlign::CENTER,
            7.5,
            Color::WHITE,
        );

        // PRESS ENTER TO STABILIZE
        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            "PRESS ENTER TO",
            vec2(-10.0, 0.0),
            geng::TextAlign::CENTER,
            5.0,
            Color::WHITE,
        );
        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            "STABILIZE",
            vec2(-10.0, -10.0),
            geng::TextAlign::CENTER,
            5.0,
            Color::WHITE,
        );

        // Icon
        self.geng.draw_2d().textured_quad(
            framebuffer,
            &self.camera,
            AABB::point(reactor_pos).extend_uniform(10.0),
            &self.assets.nuclear,
            reactor_color,
        );
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        if self.geng.window().is_key_pressed(geng::Key::Enter) {
            Some(geng::Transition::Switch(Box::new(
                crate::game::GameState::new(&self.geng, &self.assets),
            )))
        } else {
            None
        }
    }
}
