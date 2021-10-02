use geng::prelude::*;

mod game;
mod pregame;

#[derive(geng::Assets)]
pub struct Assets {
    pub nuclear: ugli::Texture,
    pub rocket: ugli::Texture,
    pub rocket_booster: ugli::Texture,
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    // Setup working directory
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("static")).unwrap();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                std::env::set_current_dir(path).unwrap();
            }
        }
    }

    // Intialize geng
    let geng = Geng::new("Unstable Asteroids");
    let assets = <Assets as geng::LoadAsset>::load(&geng, ".");

    // Run
    geng::run(
        &geng,
        geng::LoadingScreen::new(&geng, geng::EmptyLoadingScreen, assets, {
            let geng = geng.clone();
            move |assets| {
                let mut assets = assets.unwrap();
                assets.nuclear.set_filter(ugli::Filter::Nearest);
                assets.rocket.set_filter(ugli::Filter::Nearest);
                assets.rocket_booster.set_filter(ugli::Filter::Nearest);

                pregame::PregameState::new(&geng, &Rc::new(assets))
            }
        }),
    );
}
