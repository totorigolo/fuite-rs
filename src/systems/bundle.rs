use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::DispatcherBuilder,
    assets::{
        PrefabLoaderSystem,
        HotReloadStrategy,
        HotReloadSystem,
    },
};

use crate::{
    systems::*,
    states::GamePrefabData,
};

/// Bundle of all the game Systems, to avoid polluting main.rs
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {

//        builder.add(System::default(), "system_name", &[
//            "dependency_1",
//            "dependency_2",
//            "dependency_3",
//        ]);

        builder.add(PrefabLoaderSystem::<GamePrefabData>::default(), "", &[]);

        builder.add(HotReloadSystem::new(HotReloadStrategy::every(2)), "", &[]);


        builder.add(LevelManager::default(), "level_mgr_system", &[]);
        builder.add(Fps::default(), "fps_system", &[]);
        builder.add(Text::default(), "text_system", &[]);

        builder.add(PlayerInput::default(), "player_input_system", &[]);
        builder.add(CameraMoveSystem::default(), "camera_system", &[]);

        builder.add(Physics::default(), "physics_system", &[]);

//        use log::*;
//        debug!("{:?}", builder);
        Ok(())
    }
}
