use amethyst::{
    assets::{PrefabLoader, RonFormat},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::rendy::mesh::{Normal, Position, TexCoord},
    utils::scene::BasicScenePrefab,
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};

pub type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let world_prefab_handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefabs/world.ron", RonFormat, ())
        });
        let cube_prefab_handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefabs/cube.ron", RonFormat, ())
        });
        let sphere_prefab_handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefabs/sphere.ron", RonFormat, ())
        });
        world.create_entity().with(world_prefab_handle).build();
        world.create_entity().with(cube_prefab_handle).build();
        world.create_entity().with(sphere_prefab_handle).build();
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}
