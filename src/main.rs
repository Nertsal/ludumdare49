use geng::prelude::*;

mod game;

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Ludum Dare 49 - Unstable");
    let state = game::GameState::new(&geng);
    geng::run(&geng, state);
}
