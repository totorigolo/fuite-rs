use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    renderer::{DrawFlat, PosTex, Stage, DisplayConfig, Pipeline, RenderBundle},
    input::InputBundle,
    ui::{UiBundle, DrawUi},
    utils::{
        application_root_dir,
        fps_counter::FPSCounterBundle,
    },
};

mod logger;
mod states;
mod components;
mod systems;
mod config;
mod resources;

use crate::{
    states::StartingState,
    systems::bundle::GameBundle,
    config::GameConfig,
};

fn main() -> amethyst::Result<()> {
    logger::start_custom_logger(Default::default());

    let app_root = application_root_dir();
    let assets_path = format!("{}/assets/", app_root);

    let game_config_path = format!("{}/resources/game_config.ron", app_root);
    let game_config = GameConfig::load(&game_config_path);

    let display_config_path = format!("{}/resources/display_config.ron", app_root);
    let display_config = DisplayConfig::load(&display_config_path);

    let rendering_pipeline = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(game_config.void_color.clone(), 1.0)
            .with_pass(DrawFlat::<PosTex>::new())
            .with_pass(DrawUi::new()),
    );

    let bindings_config_path = format!("{}/resources/bindings_config.ron", app_root);
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(bindings_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(input_bundle)?
        .with_bundle(GameBundle)?
        .with_bundle(RenderBundle::new(rendering_pipeline, Some(display_config)))?;

    Application::build(assets_path, StartingState::default())?
        .with_resource(game_config.clone())
        .with_resource(game_config.levels)
        .build(game_data)?
        .run();

    Ok(())
}
