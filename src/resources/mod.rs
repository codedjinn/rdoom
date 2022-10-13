use crate::defs::entity_def::EntityDefintion;

use bevy::prelude::*;
use anyhow::Result;
use walkdir::WalkDir;


pub mod conversion;

pub struct GameDefinitions {
    entities: EntityDefintion
}

pub fn load_definitions(
    mut game_definitions: ResMut<GameDefinitions>
) {
    
    let dir = std::fs::read_dir("assets/entities").unwrap();

    for entry in WalkDir::new("assets/entities/*.json") {
        println!("entry {:?}", entry.unwrap().file_name());
    }
}