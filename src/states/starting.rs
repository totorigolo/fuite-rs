use amethyst::{
    prelude::*,
    assets::{
        Prefab,
        PrefabLoader,
        RonFormat,
        Handle,
        ProgressCounter,
        Completion,
    },
    ui::{
        UiCreator,
        UiFinder,
    },
};
use log::*;

use crate::states::game::{
    Game,
    GamePrefabData,
};


#[derive(Default)]
pub struct StartingState {
    load_progress: ProgressCounter,
    load_complete: bool,

    game_prefab: Option<Handle<Prefab<GamePrefabData>>>,
}

impl<'a, 'b> SimpleState<'a, 'b> for StartingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("Starting...");

        let world = data.world;

        info!("Loading UI...");
        world.exec(|mut creator: UiCreator| {
            creator.create("ui/fps.ron", &mut self.load_progress);
            creator.create("ui/loading.ron", &mut self.load_progress);
            creator.create("ui/game.ron", &mut self.load_progress);
        });

        info!("Loading scene prefab...");
        self.game_prefab = Some(world.exec(|loader: PrefabLoader<GamePrefabData>| {
            loader.load("prefab/scene.ron", RonFormat, (), &mut self.load_progress)
        }));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        match &self.load_complete {
            false => {
                match &self.load_progress.complete() {
                    Completion::Loading => Trans::None,
                    Completion::Complete => {
                        self.load_complete = true;
                        Trans::None
                    }
                    Completion::Failed => {
                        error!("Failed to load assets, exiting");
                        Trans::Quit
                    }
                }
            }
            true => {
                info!("Loading successful.");

                // Remove the waiting message
                if let Some(entity) = data.world.exec(|finder: UiFinder| finder.find("loading")) {
                    data.world.delete_entity(entity).ok();
                }

                info!("Switching to GameState...");
                Trans::Switch(Box::new(Game::new(
                    self.game_prefab.take()
                        .expect("Game Prefab loading logic error."),
                )))
            }
        }
    }
}
