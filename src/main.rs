use geng::prelude::*;

mod game;
mod game_over;
mod pregame;
mod renderer;

#[derive(Deref)]
pub struct Font {
    #[deref]
    inner: Rc<geng::Font>,
}

impl geng::LoadAsset for Font {
    fn load(geng: &Geng, path: &str) -> geng::AssetFuture<Self> {
        let geng = geng.clone();
        <Vec<u8> as geng::LoadAsset>::load(&geng, path)
            .map(move |data| {
                Ok(Font {
                    inner: Rc::new(geng::Font::new(&geng, data?)?),
                })
            })
            .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("ttf");
}

#[derive(geng::Assets)]
pub struct Sounds {
    pub asteroid_hit: geng::Sound,
    pub explosion: geng::Sound,
    pub select: geng::Sound,
    pub item_select: geng::Sound,
    pub purchase: geng::Sound,
    pub reject: geng::Sound,
}

#[derive(geng::Assets)]
pub struct Sprites {
    pub nuclear: ugli::Texture,
    pub rocket: ugli::Texture,
    pub rocket_booster: ugli::Texture,
    #[asset(path = "asteroid/*.png", range = "1..=3")]
    pub asteroids: Vec<Rc<ugli::Texture>>,
}

#[derive(geng::Assets)]
pub struct Assets {
    #[asset(path = "fonts/NF_pixels/fonts/ttf/NFPixels-Regular.ttf")]
    pub font: Font,
    pub sprites: Sprites,
    pub sounds: Sounds,
}

impl Sprites {
    fn init(&mut self) {
        self.nuclear.set_filter(ugli::Filter::Nearest);
        self.rocket.set_filter(ugli::Filter::Nearest);
        self.rocket_booster.set_filter(ugli::Filter::Nearest);
        self.asteroids.iter_mut().for_each(|texture| {
            Rc::get_mut(texture)
                .unwrap()
                .set_filter(ugli::Filter::Nearest)
        });
    }
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
                assets.sprites.init();

                pregame::PregameState::new(&geng, &Rc::new(assets))
            }
        }),
    );
}
