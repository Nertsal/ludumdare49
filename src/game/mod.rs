use geng::prelude::*;

mod circle;
mod draw;
mod handle_event;
mod player;
mod update;

use circle::*;
use player::*;

pub struct GameState {
    // Usual engine things
    geng: Geng,
    camera: geng::Camera2d,
    framebuffer_size: Vec2<f32>,

    // Actual game data
    player: Player,
}

impl GameState {
    pub fn new(geng: &Geng) -> Self {
        Self {
            // Usual engine things
            geng: geng.clone(),
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            framebuffer_size: vec2(1.0, 1.0),

            // Actual game data
            player: {
                let circle = Circle::new(Vec2::ZERO, 5.0, Color::rgb(1.0, 0.0, 0.0));
                Player::new(RigidCircle::new(circle, 10.0), 50.0, 300.0)
            },
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.update_impl(delta_time);
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw_impl(framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.hande_event_impl(event);
    }
}
