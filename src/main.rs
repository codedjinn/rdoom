
mod wad;

//use bevy::prelude::*;
use anyhow::Result;
use wad::{WadLumpType, WadColor};

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() -> Result<()> {

    let file = std::fs::File::open("assets/doom1.wad")?;

    let wad = wad::Wad::load_from_file(&file)?;

    let wad_assets = wad::WadAssets::load_from(&wad)?;
    
    let pal = wad_assets.get_default_pal();

//    println!("default pal size {}", pal.colors().len());

   // let pal = wad_result.get_palette(0);

//    let pal = wad_result.get_color_palette(0);

    //println!("PLAYPAL length {}", pal.colors.len());
    // let pal 
    //     = wad_result
    //         .lumps()
    //         .iter()
    //         .find(|&x| x.lump_type() == WadLumpType::Palette);

    // let bytes = pal.unwrap().data();

    //let slice = &bytes[0..3];
    //let r = wad::util::byte_array_4_to_int(slice);
        
    // let mut new_image = bmp::Image::new(255, 1);

    // let mut i = 0usize;
  

    // new_image.save("d:\\pal2.bmp");

    Ok(())
}

