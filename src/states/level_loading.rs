use amethyst::{
    prelude::*,
    ecs::prelude::*,
    assets::{
        Loader,
        Prefab,
        Handle,
    },
    ui::{
        UiFinder,
    },
    core::{
        Transform,
        nalgebra::{
            Vector2, Vector3,
        },
    },
    renderer::{
        PosTex,
        Material,
        MaterialDefaults,
        MeshHandle,
        HiddenPropagate,
    },
};
use log::*;

use crate::{
    resources::*,
    components::*,
    config::*,
    states::*,
};


#[derive(Clone)]
enum LoadingState {
    NeedToLoadLevels,
    LevelsLoaded,
    NeedToLoadLevel,
    LevelLoaded,
}

pub struct LevelLoadingState {
    // To pass to GameState
    scene_handle: Handle<Prefab<GamePrefabData>>,

    // Level loading related
    loading_state: LoadingState,
}

impl LevelLoadingState {
    pub fn new(scene_handle: Handle<Prefab<GamePrefabData>>) -> Self {
        LevelLoadingState {
            scene_handle,
            loading_state: LoadingState::NeedToLoadLevels,
        }
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for LevelLoadingState {
    fn on_resume(&mut self, data: StateData<GameData>) {
        self.show_loading(data.world);
        self.unload_level(data.world);
        self.loading_state = LoadingState::NeedToLoadLevel;
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        match self.loading_state.clone() {
            LoadingState::NeedToLoadLevels => {
                self.load_levels(data.world);
                self.loading_state = LoadingState::LevelsLoaded;
                Trans::None
            }
            LoadingState::LevelsLoaded => {
                self.loading_state = LoadingState::NeedToLoadLevel;
                Trans::None
            }
            LoadingState::NeedToLoadLevel => {
                {
                    let mut resource = data.world.write_resource::<LevelResource>();
                    let idx = resource.current_level.expect("current_level is None!");
                    if idx >= resource.levels.len() {
                        resource.current_level = Some(0);
                    }
                }

                self.load_level(data.world);
                self.loading_state = LoadingState::LevelLoaded;
                Trans::None
            }
            LoadingState::LevelLoaded => {
                self.hide_loading(data.world);
                Trans::Push(Box::new(GameState::new(self.scene_handle.clone())))
            }
        }
    }
}

impl LevelLoadingState {
    fn load_levels(&mut self, world: &mut World) {
        info!("Loading levels...");

        let config = world.read_resource::<LevelsConfig>();

        // Assert that there are at least one level
        if config.levels.is_empty() {
            panic!("No level were loaded, the config is corrupted.");
        }

        let mut level_resource = world.write_resource::<LevelResource>();
        level_resource.current_level = Some(config.start_level);
        level_resource.levels = config.levels.clone();

        info!("Levels loaded.");
    }

    fn load_level(&mut self, world: &mut World) {
        info!("Loading level...");

        let level = {
            let resource = world.read_resource::<LevelResource>();
            let mut idx = resource.current_level.expect("load_level: current_level is None!");
            if resource.finished { idx += 1 }
            idx %= resource.levels.len();
            let level = resource.levels[idx].clone();
            info!("=> #{}: {}", idx, level.name);
            level
        };

        for platform in &level.platforms.list {
            create_platform(world, &platform);
        }
        create_rocket(world, &level.rocket);
        for hum in &level.hums.list {
            create_hum(world, &hum);
        }

        world.write_resource::<LevelResource>().finished = false;

        info!("Level loaded.");
    }

    fn unload_level(&mut self, world: &mut World) {
        info!("Unloading level...");

        let _level = {
            let resource = world.read_resource::<LevelResource>();
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
            let rockets = world.read_storage::<Rocket>();
            for (e, _) in (&entities, &hums).join() {
                entities.delete(e).expect("Failed to delete hum.");
            }
            for (e, _) in (&entities, &colliders).join() {
                entities.delete(e).expect("Failed to delete collider.");
            }
            for (e, _) in (&entities, &rockets).join() {
                entities.delete(e).expect("Failed to delete rocket.");
            }
        }

        info!("Level unloaded.");
    }

    fn show_loading(&mut self, world: &mut World) {
        if let Some(entity) = world.exec(|finder: UiFinder| finder.find("loading")) {
            world.write_storage::<HiddenPropagate>().remove(entity)
                .expect("Failed to show the loading text.");
        }
    }

    fn hide_loading(&mut self, world: &mut World) {
        if let Some(entity) = world.exec(|finder: UiFinder| finder.find("loading")) {
            world.write_storage().insert(entity, HiddenPropagate)
                .expect("Failed to hide the loading text.");
        }
    }
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

    let mesh = create_mesh(
        world,
        generate_quadrangle_vertices(top_left, top_right, bottom_left, bottom_right),
    );
    let material = create_colour_material(world, hum.color.clone().into());

    let hum_entity = world.create_entity()
        .with(vec2_to_trans(hum.position))
        .with(hum.mass.clone())
        .with(hum.shape.clone())
        .with(Velocity::default())
        .with(mesh.clone())
        .with(material.clone())
        .with(Health(hum.health))
        .with(hum.action.clone())
        .with(PhysicForce::default());
    let hum_entity = if hum.is_bad {
        hum_entity.with(Bad)
    } else { hum_entity };
    hum_entity.build();
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

fn create_rocket(world: &mut World, rocket: &RocketConfig) {
    debug!("Creating Rocket: {:?}", rocket);

    let width = rocket.width;
    let height = rocket.height;
    let x = rocket.position.x;
    let y = rocket.position.y;

    let aabb = AABB {
        top: y + height,
        bottom: y,
        left: x - width / 2.0,
        right: x + width / 2.0,
    };

    let mesh = create_mesh(
        world,
        generate_rocket_vertices(
            aabb.left - x, aabb.bottom - y,
            aabb.right - x, aabb.top - y,
            rocket.cap,
        ),
    );
    let material = create_colour_material(world, rocket.color.clone().into());

    let mut transform = Transform::default();
    transform.set_xyz(x, y, 0.0);
    trace!("Transform={:?}", transform);

    world.create_entity()
        .with(transform)
        .with(aabb.clone())
        .with(Collider)
        .with(mesh)
        .with(material)
        .with(Health(rocket.health))
        .with(Rocket::new(rocket.min_passengers, rocket.health, aabb))
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

/// Generates vertices forming a Rocket /o/.
fn generate_rocket_vertices(left: f32, bottom: f32, right: f32, top: f32, cap: f32) -> Vec<PosTex> {
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
            tex_coord: Vector2::new(1.0, 0.8),
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
        // Triangle: /\
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(0.0, 0.8),
        },
        PosTex {
            position: Vector3::new(right, top, 0.0),
            tex_coord: Vector2::new(1.0, 0.8),
        },
        PosTex {
            position: Vector3::new((left + right) / 2.0, top + cap, 0.0),
            tex_coord: Vector2::new(0.5, 1.0),
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
