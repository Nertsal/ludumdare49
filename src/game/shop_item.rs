use super::*;

pub struct ShopItem {
    pub name: String,
    pub cost: u32,
    pub effect: Effect,
}

impl GameState {
    pub fn gen_shop_item(&self) -> ShopItem {
        ShopItem {
            name: "Stabilize".to_owned(),
            cost: 50,
            effect: Effect::HealReactor { heal: 100.0 },
        }
    }
}
