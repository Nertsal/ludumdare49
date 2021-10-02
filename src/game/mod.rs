use geng::prelude::*;

mod asteroid;
mod circle;
mod collide;
mod collision;
mod constants;
mod draw;
mod handle_event;
mod player;
mod reactor;
mod spawn;
mod update;

use asteroid::*;
use circle::*;
use constants::*;
use player::*;
use reactor::*;

pub struct GameState {
    // Usual engine things
    geng: Geng,
    camera: geng::Camera2d,
    framebuffer_size: Vec2<f32>,

    // Actual game data
    border: AABB<f32>,

    player: Player,
    reactor: Reactor,
    asteroids: Vec<Asteroid>,

    spawn_delay: f32,
    spawn_timer: f32,
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
            border: AABB::ZERO.extend_symmetric(BORDER_SIZE / 2.0),

            player: {
                let circle = Circle::new(Vec2::ZERO, PLAYER_RADIUS, PLAYER_COLOR);
                Player::new(
                    RigidCircle::new(circle, PLAYER_MASS),
                    PLAYER_SPEED,
                    PLAYER_ACCELERATION,
                )
            },
            reactor: {
                let circle = Circle::new(Vec2::ZERO, REACTOR_RADIUS, REACTOR_COLOR);
                Reactor::new(circle, REACTOR_HEALTH)
            },
            asteroids: vec![],

            spawn_delay: SPAWN_DELAY,
            spawn_timer: SPAWN_DELAY,
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
