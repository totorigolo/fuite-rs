use amethyst::{
    ecs::prelude::*,
    core::timing::Time,
    ui::{UiFinder, UiText},
    utils::fps_counter::FPSCounter,
};
#[allow(unused_imports)]
use log::*;


/// FPS updater
#[derive(Default)]
pub struct Fps {
    fps_display: Option<Entity>,
}

impl<'s> System<'s> for Fps {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, FPSCounter>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (time, fps_counter, mut ui_text, finder): Self::SystemData) {
        if let None = self.fps_display {
            if let Some(fps_entity) = finder.find("fps_text") {
                self.fps_display = Some(fps_entity);
            }
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    let fps = fps_counter.sampled_fps();
                    fps_display.text = format!("FPS: {:.0}", fps);
                }
            }
        }
    }
}
