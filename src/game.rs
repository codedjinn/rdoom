
use bevy::prelude::*;

use crate::wad::{WadAssets};

#[derive(Resource)]
pub struct GameData {
    pub wad_assets: WadAssets
}