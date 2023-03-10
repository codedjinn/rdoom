
use bevy::prelude::*;

use crate::wad::{WadAssets};

#[derive(Resource)]
pub struct GameData {
    pub wad_assets: WadAssets,
    pub player_start: Vec3,
    pub start_set: bool,
}