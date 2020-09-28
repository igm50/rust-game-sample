use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct GameState;

impl SimpleState for GameState {}

const CONFIG_DIR: &str = "config";
const CONFIG_FILE: &str = "display.ron";
const ASSETS_DIR: &str = "assets";

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join(ASSETS_DIR);
    let display_config_path = app_root.join(CONFIG_DIR).join(CONFIG_FILE);

    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
