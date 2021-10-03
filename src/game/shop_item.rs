use super::*;

#[derive(Clone)]
pub struct ShopItem {
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub effect: Effect,
}

impl GameState {
    pub fn gen_shop_item(&self) -> ShopItem {
        let possible_items = vec![
            (
                ShopItem {
                    name: "Stabilize".to_owned(),
                    description: "Is your reactor\nunstable?\nStabilize it now!".to_owned(),
                    cost: 50,
                    effect: Effect::HealReactor { heal: 100.0 },
                },
                10.0,
            ),
            (
                ShopItem {
                    name: "Explode".to_owned(),
                    description: "Explode the reactor\nto clear the screen\nfrom asteroids."
                        .to_owned(),
                    cost: 100,
                    effect: Effect::ExplodeReactor,
                },
                5.0,
            ),
        ];

        possible_items
            .choose_weighted(&mut global_rng(), |(_, weight)| *weight)
            .expect("failed to generate new shop item")
            .0
            .clone()
    }
}
