use super::*;

impl GameState {
    pub fn hande_event_impl(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => {
                match key {
                    geng::Key::F => {
                        self.is_shop_open = !self.is_shop_open;
                        self.shop_item_select = None;
                    }
                    _ => (),
                }
                if self.is_shop_open {
                    // Shop specific actions
                    match key {
                        geng::Key::A | geng::Key::Left if self.is_shop_open => {
                            self.shop_item_select(-1);
                        }
                        geng::Key::D | geng::Key::Right if self.is_shop_open => {
                            self.shop_item_select(1);
                        }
                        geng::Key::Space | geng::Key::Enter if self.is_shop_open => {
                            self.try_buy_shop_item();
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    fn shop_item_select(&mut self, delta: isize) {
        self.assets.sounds.item_select.play();
        match self.shop_item_select {
            Some(current) => {
                let mut current = current as isize + delta;
                while current < 0 {
                    current += self.shop_item_count as isize;
                }
                while current >= self.shop_item_count as isize {
                    current -= self.shop_item_count as isize;
                }
                self.shop_item_select = Some(current as usize);
            }
            None => {
                self.shop_item_select = Some(0);
            }
        }
    }

    fn try_buy_shop_item(&mut self) {
        match self.shop_item_select {
            Some(select) => {
                // Check if we have enough money
                if self.money >= self.shop_items[select].cost {
                    // Purchase
                    self.assets.sounds.purchase.play();
                    let new_item = self.gen_shop_item();
                    let purchase = std::mem::replace(&mut self.shop_items[select], new_item);
                    self.money -= purchase.cost;
                    self.apply_effect(purchase.effect);
                } else {
                    self.assets.sounds.reject.play();
                }
            }
            None => (),
        }
    }
}
