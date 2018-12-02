use amethyst::{
    ecs::prelude::*,
    assets::Loader,
    core::{
        Transform,
        nalgebra::{
            Vector2, Vector3,
        },
    },
    renderer::{PosTex, Material, MaterialDefaults, MeshHandle},
};
use log::*;

use crate::{
    resources::*,
    components::*,
    config::*,
};

///Finds shapes without meshes and creates meshes for them
#[derive(Default)]
pub struct LevelManager {
    level_just_loaded: bool,
    message_reader: Option<ReaderId<Message>>,
}

impl<'s> System<'s> for LevelManager {
    type SystemData = (
        Read<'s, LazyUpdate>,
        Read<'s, LevelResource>,
        Write<'s, MessageChannel>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.message_reader = Some(res.fetch_mut::<MessageChannel>().register_reader());
    }

    fn run(&mut self, (updater, levels, mut message_channel): Self::SystemData) {
        match levels.state.clone() {
            // The resource has just been created from Default
            // => load the levels
            LevelResourceState::JustCreated => {
                updater.exec_mut(move |world| load_levels(world));
            }
            // The config is loaded, but no level is loaded
            // => load the first level
            LevelResourceState::ConfigLoaded | LevelResourceState::ReloadNeeded => {
                updater.exec_mut(move |world| load_level(world));
                self.level_just_loaded = true;
            }
            // The config and a level are loaded
            LevelResourceState::Loaded => {
                if self.level_just_loaded.clone() {
                    let msg = Message::LevelLoaded;
                    debug!("New message: {:?}", msg);
                    message_channel.single_write(msg);
                    self.level_just_loaded = false;
                }

                // Process messages in the MessageChannel
                for message in message_channel.read(self.message_reader.as_mut().unwrap()) {
                    match message {
                        Message::ReloadLevels => {
                            updater.exec_mut(move |world| {
                                unload_level(world);
                                load_levels(world);
                            });
                        }
                        Message::ReloadLevel => {
                            updater.exec_mut(move |world| restart_level(world));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn load_levels(world: &mut World) {
    info!("Loading levels...");

    let config = world.read_resource::<LevelsConfig>();

    // For the time being, we only need one level
    assert_eq!(config.levels.len(), 1,
               "Invalid level data, there are {} levels.", config.levels.len());
    assert_eq!(config.start_level, 0,
               "Invalid level data, start_level={}.", config.start_level);

    let mut level_resource = world.write_resource::<LevelResource>();
    level_resource.current_level = Some(0);
    level_resource.levels = config.levels.clone();
    level_resource.state = LevelResourceState::ConfigLoaded;

    info!("Levels loaded!");
}

fn load_level(world: &mut World) {
    info!("Loading level...");

    let level = {
        let resource = world.read_resource::<LevelResource>();
        let idx = resource.current_level.expect("load_level: current_level is None!");
        let level = resource.levels[idx].clone();
        info!("=> #{}: {}", idx, level.name);
        level
    };

    for platform in &level.platforms.list {
        create_platform(world, &platform);
    }
    for hum in &level.hums.list {
        create_hum(world, &hum);
    }

    let mut level_resource = world.write_resource::<LevelResource>();
    level_resource.state = LevelResourceState::Loaded;
    info!("Level loaded!");
}

fn unload_level(world: &mut World) {
    info!("Unloading level...");

    let _level = {
        let resource = world.read_resource::<LevelResource>();
        if resource.state != LevelResourceState::Loaded {
            error!("unload_level: no level loaded");
            return;
        }

        let idx = resource.current_level.expect("load_level: current_level is None!");
        let level = resource.levels[idx].clone();
        info!("=> #{}: {}", idx, level.name);
        level
    };

    // TODO: Restore the camera to its location

    // Delete all Hums and static shapes
    {
        let entities = world.entities();
        let hums = world.read_storage::<HumShape>();
        let colliders = world.read_storage::<Collider>();
        for (e, _) in (&entities, &hums).join() {
            entities.delete(e).expect("Failed to delete hum.");
        }
        for (e, _) in (&entities, &colliders).join() {
            entities.delete(e).expect("Failed to delete collider.");
        }
    }

    let mut level_resource = world.write_resource::<LevelResource>();
    level_resource.state = LevelResourceState::ReloadNeeded;
}

fn restart_level(world: &mut World) {
    unload_level(world);
}

fn vec2_to_trans(pos: Vector2<f32>) -> Transform {
    let mut t = Transform::default();
    t.set_xyz(pos.x, pos.y, 0.0);
    t
}

fn create_hum(world: &mut World, hum: &HumConfig) {
    debug!("Creating Hum: {:?}", hum);

    let ref shape = hum.shape;
    let height = shape.height;
    let top = shape.top;
    let base = shape.base;

    let top_left = (-top / 2.0, height);
    let top_right = (top / 2.0, height);
    let bottom_left = (-base / 2.0, 0.0);
    let bottom_right = (base / 2.0, 0.0);

//    let border_mesh = create_mesh(
//        world,
//        generate_quadrangle_vertices(top_left, top_right, bottom_left, bottom_right),
//    );
//    let border_material = create_colour_material(world, [0.0, 0.0, 0.0, 0.0]);
    let color_mesh = create_mesh(
        world,
        generate_quadrangle_vertices(top_left, top_right, bottom_left, bottom_right),
    );
    let color_material = create_colour_material(world, hum.color.clone().into());

    world.create_entity()
        .with(vec2_to_trans(hum.position))
        .with(hum.mass.clone())
        .with(hum.shape.clone())
        .with(Velocity::default())
//        .with(border_mesh)
//        .with(border_material)
        .with(color_mesh)
        .with(color_material)
        .build();
}

fn create_platform(world: &mut World, platform: &PlatformConfig) {
    debug!("Creating Platform: {:?}", platform);

    let top = platform.aabb.top;
    let bottom = platform.aabb.bottom;
    let left = platform.aabb.left;
    let right = platform.aabb.right;
    let x = (left + right) / 2.0;
    let y = (top + bottom) / 2.0;

    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(left - x, bottom - y, right - x, top - y),
    );
    let material = create_colour_material(world, platform.color.clone().into());

    let mut transform = Transform::default();
    transform.set_xyz(x, y, 0.0);
    trace!("Transform={:?}", transform);

    world.create_entity()
        .with(transform)
        .with(platform.aabb.clone())
        .with(Collider)
        .with(mesh)
        .with(material)
        .build();
}

/// Generates vertices forming a rectangle.
/// From the Amethyst appendix_a example
fn generate_quadrangle_vertices(
    top_left: (f32, f32),
    top_right: (f32, f32),
    bottom_left: (f32, f32),
    bottom_right: (f32, f32),
) -> Vec<PosTex> {
    fn vec2to3(vec2: (f32, f32)) -> Vector3<f32> {
        Vector3::new(vec2.0, vec2.1, 0.0)
    }

    vec![
        // Triangle: |\
        PosTex {
            position: vec2to3(top_left),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        PosTex {
            position: vec2to3(bottom_left),
            tex_coord: Vector2::new(1.0, 0.0),
        },
        PosTex {
            position: vec2to3(bottom_right),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        // Triangle: \|
        PosTex {
            position: vec2to3(top_left),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        PosTex {
            position: vec2to3(bottom_right),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        PosTex {
            position: vec2to3(top_right),
            tex_coord: Vector2::new(0.0, 1.0),
        },
    ]
}

/// Generates vertices forming a rectangle.
/// From the Amethyst appendix_a example
fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
    vec![
        // Triangle: |\
        PosTex {
            position: Vector3::new(left, bottom, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        // Triangle: \|
        PosTex {
            position: Vector3::new(right, top, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
    ]
}

/// Creates a solid material of the specified colour.
/// From the Amethyst appendix_a example
fn create_colour_material(world: &World, colour: [f32; 4]) -> Material {
    // TODO: optimize
    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

/// Converts a vector of vertices into a mesh.
/// From the Amethyst appendix_a example
fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}
