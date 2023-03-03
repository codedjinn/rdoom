// use super::{
//     FileLump,
//     Wad,
//     WadHeader,
//     WadLump,
//     WadLumpType,
//     WadType,
//     WadColor,
//     WadPalette,
//     WadAssets,
//     WadThing,
//     WadLineDef,
//     WadVertex,
//     WadVertexes,
//     WadSector,
//     WadMap,
//     WadMapLevel
// };
// use super::{util};
// use super::{doom1};

use super::{
    WadColor,
    WadPalette
};

use std::str::FromStr;

// use std::io::{Read, Seek, SeekFrom};
// use std::fs::File;
use super::{
    Wad, WadLumpType, WadLevel
};
use anyhow::Result;
use bevy::utils::tracing::level_filters;

use super::WadAssets;

impl WadAssets {
    pub fn load_from(wad: &Wad) -> Result<WadAssets> {
        
        let mut assets = WadAssets::new();

        let mut i = 0;
        while i < wad.lumps().len() {
            let lump = wad.get_lump(i);

            // support map markers first
            match lump.lump_type {
                WadLumpType::Marker => {
                    println!("Marker");
                    let level_result = WadLevel::from_str(lump.name());
                    match level_result {
                        Ok(level) => {
                            println!("level {:?}", level);

                            // hard-coded goodness :P
                            
                        }
                        Err(_) => { /* figure out what to do here */}
                    };
                }
                WadLumpType::Palette => {
                    println!("Palette");
                    let result = resolve_palettes(&wad);
                    match result {
                        Ok(pals) => assets.set_palettes(pals),
                        Err(_) => { panic!("Can't generate content without palettes"); }
                    };
                }
                _ => {}
            };
        }
        return Ok(assets);
    }
}

fn resolve_palettes(wad: &Wad) -> Result<Vec<WadPalette>> {
    let playpal 
        = wad
            .lumps()
            .iter()
            .find(|&x| x.lump_type() == WadLumpType::Palette);

    let mut result: Vec<WadPalette> = Vec::new();

    if playpal.is_some() {
        let data = playpal.unwrap().data();
        let pal_count = data.len() / 3usize / 255usize;
        for _ in 0..pal_count {
            let mut new_pal = WadPalette::new();
            for i in 0..255 {
                let offset = i * 3usize;
                
                new_pal.add(WadColor {
                    r: data[offset],
                    g: data[offset+1],
                    b: data[offset+2],
                });
            }
            result.push(new_pal);    
        }
    }
    return Ok(result);
}


// impl WadAssets {
    

//     pub fn load_from(wad: &Wad) -> Result<WadAssets> {

//     //     let palettes = WadAssets::resolve_palette(wad)?;
//     //     let things = WadAssets::resolve_things(wad)?;
//     //   //  let pictures = picture::resolve_pictures(wad, &palettes[0])?;
//     //     let line_defs = WadAssets::resolve_linedefs(wad)?;
//     //     let vertexes = WadAssets::resolve_vertexes(wad)?;
//     //     let maps = WadAssets::resolve_maps(wad)?;
    
//     //     let sectorsResult = WadAssets::resolve_sectors(wad);
//     //     if sectorsResult.is_err() {
//     //         panic!("No sectors found in WAD!");
//     //     }
//     //     let sectors = sectorsResult.ok().unwrap();

//     //     return Ok(WadAssets {
//     //         palettes,
//     //         pictures: Vec::new(),
//     //         things,
//     //         line_defs,
//     //         vertexes,
//     //         sectors,
//     //         maps
//     //     });

//         //return Ok(Wad);
//     }

//     fn resolve_maps(wad: &Wad) -> Result<Vec<WadMap>> {

//         let markers:Vec<_>
//             = wad
//                 .lumps()
//                 .iter()
//                 .filter(|&x| x.lump_type() == WadLumpType::Marker)
//                 .collect();

//         let mut result:Vec<WadMap> = Vec::new();

//         let mut map:std::collections::HashMap<&str, WadMapLevel> = std::collections::HashMap::new();
//         map.insert("E1M1", WadMapLevel::E1M1);
//         map.insert("E1M2", WadMapLevel::E1M2);
//         map.insert("E1M3", WadMapLevel::E1M3);
//         map.insert("E1M4", WadMapLevel::E1M4);
//         map.insert("E1M5", WadMapLevel::E1M5);
//         map.insert("E1M6", WadMapLevel::E1M6);
//         map.insert("E1M7", WadMapLevel::E1M7);
//         map.insert("E1M8", WadMapLevel::E1M8);
//         map.insert("E1M9", WadMapLevel::E1M9);

//         if !markers.is_empty() {
//             for marker in markers {
//                 if map.contains_key(marker.name()) {
//                     let v = map.get(marker.name()).unwrap();
//                     result.push(WadMap {
//                         level: v.clone(),
//                         index: marker.marker_index as usize,
//                     })
//                 }
//             }
//         }

//         return Ok(result);
//     }
    
//     fn resolve_vertexes(wad: &Wad) -> Result<Vec<WadVertexes>> {
//         let vertexes:Vec<_>
//             = wad
//                 .lumps()
//                 .iter()
//                 .filter(|&x| x.lump_type() == WadLumpType::Vertexes)
//                 .collect();

//         let mut result:Vec<WadVertexes> = Vec::new();
//         if !vertexes.is_empty() {
//             for vert in vertexes {
//                 let mut new_verts = Vec::new();
//                 let data = vert.data();
                
//                 let mut offset = 0;
//                 while offset < data.len() {
//                     let vertex = WadVertex {
//                         x: util::from_2_bytes_to_int(&data[offset..offset+2]),
//                         y: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
//                         index: vert.marker_index as usize
//                     };
//                     new_verts.push(vertex);
//                     offset = offset + 4;
//                 }
//                 result.push(WadVertexes { 
//                     verts: new_verts, 
//                     index: vert.marker_index as usize
//                 });
//             }
//         }
//         println!("vertexes count {}", result.len());
//         return Ok(result);
//     }

//     fn resolve_sectors(wad: &Wad) -> Result<Vec<WadSector>> {
//         let line_defs:Vec<_>
//             = wad
//                 .lumps()
//                 .iter()
//                 .filter(|&x| x.lump_type() == WadLumpType::Sectors)
//                 .collect();

//         let mut result:Vec<WadSector> = Vec::new();

//         if !line_defs.is_empty() {
//             for line_def in line_defs {
//                 let data = line_def.data();
//                 let mut offset = 0;
//                 while offset < data.len() {
//                     let new_item = WadSector {
//                         floor_height: util::from_2_bytes_to_int(&data[offset..offset+2]),
//                         ceiling_height: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
//                         floor_tex_name: util::from_bytes_to_string(&data[offset+4..offset+12]),
//                         ceiling_tex_name: util::from_bytes_to_string(&data[offset+12..offset+20]),
//                         light_level: util::from_2_bytes_to_int(&data[offset+20..offset+22]),
//                         sector_type: util::from_2_bytes_to_int(&data[offset+22..offset+24]),
//                         tag: util::from_2_bytes_to_int(&data[offset+24..offset+26])
//                     };
//                     result.push(new_item);
//                     offset = offset + 26;
//                 }
//             }
//         }
//         return Ok(result);
//     }


//     // fn resolve<T, F: Fn(&[u8]) -> usize>(wad: &Wad, lump_type: WadLumpType, func: F) -> Result<Vec<T>> {

//     //     let data = [0u8; 0];

//     //     let result = func(&data[0..1]);
//     //     println!("{}", result);
//     //     return Ok(Vec::new());
//     // }

//     fn resolve_linedefs(wad: &Wad) -> Result<Vec<WadLineDef>> {
//         let line_defs:Vec<_>
//             = wad
//                 .lumps()
//                 .iter()
//                 .filter(|&x| x.lump_type() == WadLumpType::LineDefs)
//                 .collect();

//         let mut result:Vec<WadLineDef> = Vec::new();

//         if !line_defs.is_empty() {
//             for line_def in line_defs {
//                 let data = line_def.data();
//                 let mut offset = 0;
//                 while offset < data.len() {
//                     let new_item = WadLineDef {
//                         start: util::from_2_bytes_to_int(&data[offset..offset+2]),
//                         end: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
//                         flags: util::from_2_bytes_to_int(&data[offset+4..offset+6]),
//                         special_type: util::from_2_bytes_to_int(&data[offset+6..offset+8]),
//                         sector_tag: util::from_2_bytes_to_int(&data[offset+8..offset+10]),
//                         right_side_def: util::from_2_bytes_to_int(&data[offset+10..offset+12]),
//                         left_side_def: util::from_2_bytes_to_int(&data[offset+12..offset+14]),
//                         index: line_def.marker_index as usize
//                     };
//                     result.push(new_item);
//                     offset = offset + 14;
//                 }
//             }
//         }
//         return Ok(result);
//     }

//     fn resolve_things(wad: &Wad) -> Result<Vec<WadThing>> {
//         let things:Vec<_>
//             = wad
//                 .lumps()
//                 .iter()
//                 .filter(|&x| x.lump_type() == WadLumpType::Things)
//                 .collect();

//         let mut result:Vec<WadThing> = Vec::new();

//         if !things.is_empty() {
//             for thing in things {
//                 let data = thing.data();
//                 //let mut count = data.len();
//                 let mut offset = 0;
//                 while offset < data.len() {
//                     let thing = WadThing {
//                         x: util::from_2_bytes_to_int(&data[offset..offset+2]),
//                         y: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
//                         rot: util::from_2_bytes_to_int(&data[offset+4..offset+6]),
//                         type_id: util::from_2_bytes_to_int(&data[offset+6..offset+8]),
//                         flags: util::from_2_bytes_to_int(&data[offset+8..offset+10]),
//                     };
//                     result.push(thing);

//                     offset = offset + 10;
//                   //  count = count - 10;
//                 }

//             }
//         }
//         return Ok(result);
//     }


// }
