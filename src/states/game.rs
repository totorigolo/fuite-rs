use amethyst::{
    prelude::*,
    assets::{
        Handle, Prefab,
    },
    renderer::{
        WindowMessages, PosTex,
    },
    input::{
        is_close_requested,
        is_key_down,
    },
    utils::scene::BasicScenePrefab,
    winit::VirtualKeyCode,
};
use log::*;


pub type GamePrefabData = BasicScenePrefab<Vec<PosTex>>;

pub struct Game {
    scene_handle: Handle<Prefab<GamePrefabData>>,
}

impl Game {
    pub fn new(scene_handle: Handle<Prefab<GamePrefabData>>) -> Self {
        Game { scene_handle }
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("Game state starting...");

        let world = data.world;

//        hide_cursor(world);

        world.create_entity().with(self.scene_handle.clone()).build();

        info!("Game state initialization success!");
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    info!("Escape pressed, bye!");
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );
                Trans::None
            }
        }
    }
}

#[allow(dead_code)]
fn hide_cursor(world: &mut World) {
    world
        .write_resource::<WindowMessages>()
        .send_command(|win| win.hide_cursor(true));

    info!("Cursor hidden.");
}
