use crate::definitions::entity_definition::EntityDefintion;

use bevy::prelude::*;
use anyhow::Result;
use walkdir::{WalkDir, DirEntry};

pub mod conversion;

#[derive(Resource)]
pub struct GameDefinitions {
    entities: Vec<EntityDefintion>
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameDefinitions { entities: Vec::new()})
           .add_startup_system(load_definitions);        
    }
}

pub fn load_definitions(
    mut game_definitions: ResMut<GameDefinitions>
) {
    let dir = std::fs::read_dir("assets/entities").unwrap();

    // Change to logging
    println!("Loading definitions...");

    for entry in WalkDir::new("assets/entities/") {
        if entry.is_err() {
            // TODO: this should be added to a logger
            println!("Error while parsing assets directoy {:?}", entry.err());
            continue;
        }
        
        let item = entry.unwrap();
        if item.file_type().is_file() {
            println!("parsing file");
            let path = std::path::Path::new(item.file_name());
            println!("path {:?}", path);
        } else {
            println!("Directory: {:?}", item.path());
        }        
    }
}
