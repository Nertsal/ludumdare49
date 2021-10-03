use geng::prelude::*;

mod asteroid;
mod circle;
mod collide;
mod collision;
mod constants;
mod draw;
mod effect;
mod handle_event;
mod particle;
mod player;
mod reactor;
mod shop_item;
mod spawn;
mod update;

use asteroid::*;
use circle::*;
use constants::*;
use effect::*;
use particle::*;
use player::*;
use reactor::*;
use shop_item::*;

use crate::{renderer::Renderer, Assets};

pub struct GameState {
    // Usual engine things
    geng: Geng,
    assets: Rc<Assets>,
    renderer: Renderer,
    camera: geng::Camera2d,
    framebuffer_size: Vec2<f32>,

    // Actual game data
    time_scale: f32,
    score: u32,
    difficulty: f32,
    border: AABB<f32>,
    transition_delay: f32,

    player: Player,
    reactor: Reactor,
    asteroids: Vec<Asteroid>,

    particle_queue: Vec<ParticleGroup>,
    particles: Vec<Particle>,

    spawn_delay: f32,
    spawn_timer: f32,

    money: u32,
    is_shop_open: bool,
    shop_item_count: usize,
    shop_items: Vec<ShopItem>,
    shop_item_select: Option<usize>,
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        let mut state = Self {
            // Usual engine things
            geng: geng.clone(),
            assets: assets.clone(),
            renderer: Renderer::new(geng),
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 90.0,
            },
            framebuffer_size: vec2(1.0, 1.0),

            // Actual game data
            time_scale: 1.0,
            score: 0,
            difficulty: 0.0,
            border: AABB::ZERO.extend_symmetric(BORDER_SIZE / 2.0),
            transition_delay: 2.0,

            player: {
                let circle = Circle::new(
                    vec2(0.0, REACTOR_RADIUS + PLAYER_RADIUS),
                    0.0,
                    PLAYER_RADIUS,
                    Color::WHITE,
                );
                Player::new(
                    RigidCircle::new(circle, PLAYER_MASS),
                    PLAYER_LINEAR_SPEED,
                    PLAYER_LINEAR_ACCELERATION,
                    PLAYER_ROTATIONAL_SPEED,
                    PLAYER_ROTATIONAL_ACCELERATION,
                )
            },
            reactor: {
                let circle = Circle::new(Vec2::ZERO, 0.0, REACTOR_RADIUS, REACTOR_COLOR);
                Reactor::new(circle, REACTOR_HEALTH)
            },
            asteroids: vec![],

            particle_queue: vec![],
            particles: vec![],

            spawn_delay: START_SPAWN_DELAY,
            spawn_timer: START_SPAWN_DELAY,

            money: 0,
            is_shop_open: false,
            shop_item_count: 3,
            shop_items: vec![],
            shop_item_select: None,
        };
        for _ in 0..state.shop_item_count {
            let shop_item = state.gen_shop_item();
            state.shop_items.push(shop_item);
        }
        state
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

    fn transition(&mut self) -> Option<geng::Transition> {
        if self.transition_delay > 0.0 {
            return None;
        }

        let game_over = crate::game_over::GameOverState::new(&self.geng, &self.assets, self.score);
        Some(geng::Transition::Switch(Box::new(game_over)))
    }
}
