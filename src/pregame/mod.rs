use geng::prelude::*;

use crate::Assets;

pub struct PregameState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: geng::Camera2d,
    transition: bool,
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
            transition: false,
        }
    }
}

impl geng::State for PregameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let framebuffer_size = framebuffer.size().map(|x| x as f32);

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
        let font = &self.assets.font;
        font.draw(
            framebuffer,
            &self.camera,
            "Ludum Dare 49 - Unstable",
            vec2(0.0, -24.5),
            geng::TextAlign::CENTER,
            2.0,
            Color::WHITE,
        );

        // Unstable Asteroids
        font.draw(
            framebuffer,
            &self.camera,
            "Unstable Asteroids",
            vec2(0.0, 15.0),
            geng::TextAlign::CENTER,
            7.5,
            Color::WHITE,
        );

        // PRESS ENTER TO STABILIZE
        font.draw(
            framebuffer,
            &self.camera,
            "PRESS ENTER TO",
            vec2(-10.0, 0.0),
            geng::TextAlign::CENTER,
            5.0,
            Color::WHITE,
        );
        font.draw(
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

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown {
                key: geng::Key::Enter,
            } => {
                self.transition = true;
                self.assets.sounds.select.play();
            }
            _ => (),
        }
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        if !self.transition {
            return None;
        }
        self.transition = false;

        let game_state = crate::game::GameState::new(&self.geng, &self.assets);
        Some(geng::Transition::Push(Box::new(game_state)))
    }
}
