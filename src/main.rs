mod wad_parser;
mod array_util;
mod resources;

#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use wad_parser::{wad::Wad};

struct RawWadResource {
    wad: Option<Wad>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RawWadResource { wad: None })
        //.add_startup_system(load_wad_system.chain(parse_wad))
        //.add_startup_system(parse_wad)
        .add_system(handle_keys)
        
        .run();
}

fn handle_keys(
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::F1) {
        std::process::exit(0);
    }
}


// fn parse_wad(
//     wad_resource: Res<WadResource>
// ) {
//     println!("--- parse_wad ---");

//     let wad = wad_resource.wad.as_ref().unwrap();

//     let palette_result = wad.get_by_name("PLAYPAL");
//     if palette_result.is_none() {
//         panic!("Can't create graphics without a paltte");
//     }

//     let palette_data = palette_result.unwrap().lump().data().get_bytes();
    
//     let mut palette: Vec<Color> = Vec::new();

//     let mut index = 0usize;
//     while index < palette_data.len() {
//         let r = palette_data[index];
//         let g = palette_data[index + 1];
//         let b = palette_data[index + 2];
//         palette.push(Color::from(r, g, b));
//         index = index + 3;
//     }

//     let playa = wad.get_by_name("PLAYA1");
//     let bytes = playa.unwrap().lump().data().get_bytes();

//     // get int values by supplying 4 bytes
//     let width = wad_parser::convert::u8ref2_to_u32(&bytes[0..2]);
//     let height = wad_parser::convert::u8ref2_to_u32(&bytes[2..4]);
//     let left = wad_parser::convert::u8ref2_to_u32(&bytes[4..6]);
//     let top = wad_parser::convert::u8ref2_to_u32(&bytes[6..8]);

//     let size = (width * height) as usize;
//     let mut pixel_data: Vec<u8> = Vec::with_capacity(size);
//     for _ in 0..size {
//         pixel_data.push(255);
//     }

//     for col in 0..width - 1 {
//         let pointer_index = ((col * 4) + 8) as usize;
//         let mut pointer = wad_parser::convert::u8ref_to_u32(&bytes[pointer_index..pointer_index+4]) as usize;

//         loop {            
//             let row = bytes[pointer];
            
//             pointer = pointer + 1;
//             let postHeight = bytes[pointer];

//             if (row != 255 && postHeight != 255) {
//                 pointer = pointer + 1;

//                 for i in 0..postHeight {
//                     if row + i < height as u8 && pointer < bytes.len() - 1 {

//                         pointer = pointer + 1;
                        
//                         let pixel_index = (row as u32 + i as u32) * width as u32 + col as u32;                        
//                         pixel_data[pixel_index as usize] = bytes[pointer];
//                     }
//                 }

//                 pointer = pointer + 1;
//             }
//             else {
//                 break;
//             }
//             if pointer < bytes.len() - 1 {
//                 break;
//             }
//             pointer = pointer + 1;
//             if bytes[pointer] != 255 {
//                 break;
//             }
//         }

//         let mut bmp = bmp::Image::new(width, height);


//         for y in 0..height - 1 {
//             for x in 0..width - 1 {
//                 let index = ((y * width) + x) as usize;
//                 let nn = pixel_data[index];
//                 if nn == 255 {
//                     continue;
//                 }
//                 let value = palette[nn as usize];
//                 let pixel = bmp::Pixel::new(value.r, value.g, value.b);
//                 bmp.set_pixel(x, y, pixel);
//             }
//         }
//     }
//     ///let mut col_array: Vec<u32> = Vec::with_capacity(width as usize);
//     // let mut pointer:usize = 0;
//     // for i in 0..width {
//     //     let mut row_start = 0;
//     //     while row_start != 255 {
//     //         row_start = bytes[pointer];
//     //         if row_start == 255 {
//     //             break;
//     //         }
//     //         let pixel_count = bytes[row_start as usize];
//     //         pointer = pointer + 1;

//     //         for j in 0..pixel_count {
//     //             let pixel_val = bytes[pointer];
//     //             let y = j + row_start;
//     //             let x = i;
//     //             pixel_data.push(pixel_val as u32);
//     //         }
//     //     }

//     //     pointer = pointer + 1;
//     // }    

//     println!("DONE EXTRACTING DATA");

// }

fn load_wad_system(
    mut wad_resource: ResMut<RawWadResource>
) {
    // TODO:
    // Console prints should go to log file later
    println!("Loading WAD...");

    let cur_dir = std::env::current_dir().expect("Cannot resolve current directory");
    
    let cur_dir_as_str = cur_dir
        .as_os_str()
        .to_str()
        .expect("Couldn't convert OsStr to str");

    let full_path = format!("{}\\assets\\{}", cur_dir_as_str, "doom1.wad");

    println!("From path, {}", full_path);

    // from_path panics for any errors, no need to check
    wad_resource.wad = Some(Wad::from_path(full_path));
    
    println!("WAD loaded");
}
