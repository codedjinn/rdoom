use bevy::prelude::*;

#[derive(Clone)]
pub enum GameState {
    None,
    GeneratingAssets,
    Start,
    Play,
    Pause
}

pub struct DoomGame {
    state: GameState
}

impl DoomGame {
    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }
    pub fn get_state(&self) -> &GameState { &self.state }

    fn new() -> Self {
        DoomGame {
            state: GameState::None
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {

    fn build(&self, app: &mut App) {
        app.insert_resource(DoomGame::new());
    }

}