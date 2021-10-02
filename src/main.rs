use geng::prelude::*;

mod game;

#[derive(geng::Assets)]
pub struct Assets {
    pub nuclear: ugli::Texture,
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
    let geng = Geng::new("Ludum Dare 49 - Unstable");
    let assets = <Assets as geng::LoadAsset>::load(&geng, ".");

    // Run
    geng::run(
        &geng,
        geng::LoadingScreen::new(&geng, geng::EmptyLoadingScreen, assets, {
            let geng = geng.clone();
            move |assets| {
                let assets = assets.unwrap();
                game::GameState::new(&geng, &Rc::new(assets))
            }
        }),
    );
}
