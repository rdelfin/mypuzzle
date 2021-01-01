use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.init_camera(world);
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

impl GameState {
    fn init_camera(&self, world: &mut World) {
        world
            .create_entity()
            .with(Camera::standard_3d(1024., 768.))
            .build();
    }
}
