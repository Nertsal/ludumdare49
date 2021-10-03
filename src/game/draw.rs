use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.framebuffer_size = framebuffer_size;
        let camera_view = camera_view(&self.camera, framebuffer_size);

        // Draw player
        // Rocket
        self.draw_textured_circle(
            framebuffer,
            &self.player.rigid_circle.circle,
            Some(&self.assets.sprites.rocket),
        );
        // Rocket booster
        if self.player.is_accelerating {
            self.draw_textured_circle(
                framebuffer,
                &self.player.rigid_circle.circle,
                Some(&self.player.booster_keyframes[self.player.current_keyframe]),
            );
        }

        // Draw asteroids
        for asteroid in &self.asteroids {
            self.draw_textured_circle(
                framebuffer,
                &asteroid.rigid_circle.circle,
                Some(&asteroid.texture),
            );
        }

        // Draw particles
        for particle in &self.particles {
            self.draw_textured_circle(
                framebuffer,
                &particle.rigid_circle.circle,
                Some(&particle.texture),
            );
        }

        // Draw reactor
        self.draw_textured_circle(
            framebuffer,
            &self.reactor.circle,
            Some(&self.assets.sprites.nuclear),
        );

        // Reactor health
        self.assets.font.draw(
            framebuffer,
            &self.camera,
            "Reactor Stability",
            vec2(camera_view.x_min + 3.0, camera_view.y_max - 5.0),
            geng::TextAlign::LEFT,
            3.5,
            Color::WHITE,
        );

        // Draw reactor health
        let bar_position = vec2(camera_view.x_min + 3.0, camera_view.y_max - 8.0);
        let bar_width = 20.0;
        let bar_height = 2.0;
        let bar_aabb = AABB::point(bar_position).extend_positive(vec2(bar_width, bar_height));
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            bar_aabb,
            Color::rgb(0.0, 0.3, 0.0),
        );
        let offset = 0.5;
        let health_aabb = bar_aabb.extend_uniform(-offset).extend_positive(vec2(
            (self.reactor.health / self.reactor.max_health - 1.0) * (bar_width - offset),
            0.0,
        ));
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            health_aabb,
            Color::rgb(0.0, 0.7, 0.0),
        );

        // Money
        self.assets.font.draw(
            framebuffer,
            &self.camera,
            &format!("Dust: {}", self.money),
            vec2(camera_view.x_min + 3.0, camera_view.y_max - 15.0),
            geng::TextAlign::LEFT,
            4.0,
            Color::WHITE,
        );

        // Score
        self.assets.font.draw(
            framebuffer,
            &self.camera,
            &format!("SCORE: {}", self.score),
            vec2(camera_view.x_max - 20.0, camera_view.y_max - 5.0),
            geng::TextAlign::LEFT,
            4.0,
            Color::WHITE,
        );

        if self.is_shop_open {
            self.draw_shop(framebuffer);
        }
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

    fn draw_shop(&self, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let camera_view = camera_view(&self.camera, framebuffer_size);

        // Draw a panel
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            camera_view,
            Color::rgba(0.0, 0.0, 0.0, 0.5),
        );

        // Draw shop items
        let item_width = 15.0;
        let item_height = 20.0;
        let spacing = 5.0;
        let x_min = -(self.shop_item_count as f32) / 2.0 * item_width
            - (self.shop_item_count as f32 - 1.0) / 2.0 * spacing;
        let y_min = -item_height / 2.0;
        let mut shop_item_aabb =
            AABB::point(vec2(x_min, y_min)).extend_positive(vec2(item_width, item_height));

        for i in 0..self.shop_item_count {
            // Draw shop item
            let shop_item = &self.shop_items[i];
            let is_selected = self
                .shop_item_select
                .map(|select| select == i)
                .unwrap_or_default();
            let can_afford = self.money >= shop_item.cost;
            self.draw_shop_item(
                framebuffer,
                shop_item,
                shop_item_aabb,
                is_selected,
                can_afford,
            );

            shop_item_aabb = shop_item_aabb.translate(vec2(item_width + spacing, 0.0));
        }
    }

    fn draw_shop_item(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        shop_item: &ShopItem,
        shop_item_aabb: AABB<f32>,
        is_selected: bool,
        can_afford: bool,
    ) {
        // Panel
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            shop_item_aabb,
            Color::rgba(0.0, 0.0, 0.0, 0.7),
        );

        // Outline
        let (width, color) = if is_selected {
            (0.2, SHOP_ITEM_SELECTED_COLOR)
        } else {
            (0.1, SHOP_ITEM_COLOR)
        };
        self.draw_aabb_outline(framebuffer, shop_item_aabb, vec2(width, width), color);

        // Name
        self.assets.font.draw(
            framebuffer,
            &self.camera,
            &shop_item.name,
            vec2(shop_item_aabb.center().x, shop_item_aabb.y_max - 3.0),
            geng::TextAlign::CENTER,
            3.0,
            Color::WHITE,
        );

        // Description
        self.assets.font.draw(
            framebuffer,
            &self.camera,
            &shop_item.description,
            vec2(shop_item_aabb.center().x, shop_item_aabb.y_min + 10.0),
            geng::TextAlign::CENTER,
            2.0,
            Color::WHITE,
        );

        // Cost
        let color = if can_afford { Color::WHITE } else { Color::RED };
        self.assets.font.draw(
            framebuffer,
            &self.camera,
            &format!("Cost: {}", shop_item.cost),
            vec2(shop_item_aabb.center().x, shop_item_aabb.y_min + 2.0),
            geng::TextAlign::CENTER,
            3.0,
            color,
        );
    }

    fn draw_aabb_outline(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        aabb: AABB<f32>,
        width: Vec2<f32>,
        color: Color<f32>,
    ) {
        // Top and bottom
        let bottom = AABB::point(aabb.bottom_left() - vec2(width.x, 0.0))
            .extend_positive(vec2(aabb.width() + width.x * 2.0, width.y));
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            bottom.translate(vec2(0.0, -width.y)),
            color,
        );
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            bottom.translate(vec2(0.0, aabb.height())),
            color,
        );

        // Left and right
        let left = AABB::point(aabb.bottom_left()).extend_positive(vec2(width.x, aabb.height()));
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            left.translate(vec2(-width.x, 0.0)),
            color,
        );
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            left.translate(vec2(aabb.width(), 0.0)),
            color,
        );
    }
}

fn camera_view(camera: &geng::Camera2d, framebuffer_size: Vec2<f32>) -> AABB<f32> {
    let vertical_fov = camera.fov;
    let horizontal_fov = framebuffer_size.x * vertical_fov / framebuffer_size.y;
    AABB::ZERO.extend_symmetric(vec2(horizontal_fov, vertical_fov) / 2.0)
}
