
mod wad;

//use bevy::prelude::*;
use anyhow::Result;
use wad::WadLumpType;

fn main() -> Result<()> {

    let file = std::fs::File::open("assets/doom1.wad")?;

    let wad_result = wad::Wad::load_from_file(&file)?;

    let pal 
        = wad_result
            .file_lumps()
            .iter()
            .find(|&x| x.lump_type() == WadLumpType::Palette);
        
    
    Ok(())
}

