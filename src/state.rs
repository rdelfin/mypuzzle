use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        _event: StateEvent,
    ) -> SimpleTrans {
        Trans::None
    }
}
