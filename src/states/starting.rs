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
    },
};
use log::*;

use crate::{
    states::{
        LevelLoadingState,
        GamePrefabData,
    },
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
            creator.create("ui/loading.ron", &mut self.load_progress);
            creator.create("ui/fps.ron", &mut self.load_progress);
            creator.create("ui/game.ron", &mut self.load_progress);
        });

        info!("Loading scene prefab...");
        self.game_prefab = Some(world.exec(|loader: PrefabLoader<GamePrefabData>| {
            loader.load("prefab/scene.ron", RonFormat, (), &mut self.load_progress)
        }));
    }

    fn update(&mut self, _: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        match &self.load_complete {
            false => {
                match &self.load_progress.complete() {
                    Completion::Loading => {},
                    Completion::Complete => self.load_complete = true,
                    Completion::Failed => {
                        error!("Failed to load assets, exiting.");
                        return Trans::Quit
                    }
                };
                Trans::None
            }
            true => {
                info!("Loading successful.");

                info!("Switching to LevelLoadingState...");
                Trans::Switch(Box::new(LevelLoadingState::new(
                    self.game_prefab.take()
                        .expect("Game Prefab loading logic error."),
                )))
            }
        }
    }
}
