// use super::{doom1};
use super::{util};
use super::{
    WadMap,
    WadColor,
    WadPalette,
    WadVertex,
    WadThing,
    WadLineDef,
    WadSideDef,
    WadSeg,
    WadSSector,
    WadNode,
    WadSector,
    WadBBox
};

use std::str::FromStr;

// use std::io::{Read, Seek, SeekFrom};
// use std::fs::File;
use super::{
    Wad, WadLumpType, WadLevel
};
use anyhow::Result;

use super::WadAssets;

impl WadAssets {
    pub fn load_from(wad: &Wad) -> Result<WadAssets> {
        
        let mut assets = WadAssets::new();
        
        let mut maps:Vec<WadMap> = Vec::new();

        let mut i = 0;
        while i < wad.lumps().len() {
            let lump = wad.get_lump(i);

            match lump.lump_type {
                WadLumpType::Marker => {
                    let level_result = WadLevel::from_str(lump.name());
                    match level_result {
                        Ok(level) => {
                            println!("level {:?}", level);

                            // hard-coded goodness :P
                            i = i + 1;
                            println!("name {}, size {}", wad.get_lump(i).name(), wad.get_lump(i).data().len());
                            let things = resolve_things(wad.get_lump(i).data());
                            i = i + 1;
                            println!("name {}, size {}", wad.get_lump(i).name(), wad.get_lump(i).data().len());
                            let line_defs:Vec<WadLineDef> = resolve_linedefs(wad.get_lump(i).data());
                            i = i + 1;
                            println!("name {}, size {}", wad.get_lump(i).name(), wad.get_lump(i).data().len());
                            let side_defs:Vec<WadSideDef> = resolve_sidedefs(wad.get_lump(i).data());
                            i = i + 1;
                            println!("name {}, size {}", wad.get_lump(i).name(), wad.get_lump(i).data().len());
                            let vertexes:Vec<WadVertex> = resolve_vertexes(wad.get_lump(i).data());
                            i = i + 1;
                            println!("name {}, size {}", wad.get_lump(i).name(), wad.get_lump(i).data().len());
                            let segs:Vec<WadSeg> = resolve_segs(wad.get_lump(i).data());
                            i = i + 1;
                            println!("name {}, size {}", wad.get_lump(i).name(), wad.get_lump(i).data().len());
                            let ssectors:Vec<WadSSector> = resolve_ssectors(wad.get_lump(i).data());
                            i = i + 1;
                            println!("name {}, size {}", wad.get_lump(i).name(), wad.get_lump(i).data().len());
                            let nodes:Vec<WadNode> = resolve_nodes(wad.get_lump(i).data());
                            i = i + 1;
                            println!("name {}, size {}", wad.get_lump(i).name(), wad.get_lump(i).data().len());
                            let sectors:Vec<WadSector> = resolve_sectors(wad.get_lump(i).data());
                            // don't care about Reject or Blockmap
                            i = i + 2;                            
                            maps.push(WadMap {
                                level,
                                things,
                                line_defs,
                                side_defs,
                                vertexes,
                                segs,
                                ssectors,
                                nodes,
                                sectors
                            });
                        }
                        Err(_) => { /* figure out what to do here */}
                    };
                }
                WadLumpType::Palette => {
                    println!("Palette");
                    let result = resolve_palettes(lump.data());
                    assets.set_palettes(result);
                }
                _ => {}
            };
            i = i + 1;
        }
        assets.maps = Some(maps);
        return Ok(assets);
    }
}

fn resolve_palettes(data: &Vec<u8>) -> Vec<WadPalette> {
    let mut result: Vec<WadPalette> = Vec::new();

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
    return result;
}

fn resolve_vertexes(data: &Vec<u8>) -> Vec<WadVertex> {
    let mut result:Vec<WadVertex> = Vec::new();
    let mut offset = 0;
    while offset < data.len() {
        let vertex = WadVertex {
            x: util::from_2_bytes_to_int(&data[offset..offset+2]),
            y: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
        };
        result.push(vertex);
        offset = offset + 4;
    }            
    return result;
}

fn resolve_things(data: &Vec<u8>) -> Vec<WadThing> {
    let mut result:Vec<WadThing> = Vec::new();

    let mut offset = 0;
    while offset < data.len() {
        let thing = WadThing {
            x: util::from_2_bytes_to_uint(&data[offset..offset+2]),
            y: util::from_2_bytes_to_uint(&data[offset+2..offset+4]),
            rot: util::from_2_bytes_to_uint(&data[offset+4..offset+6]),
            type_id: util::from_2_bytes_to_uint(&data[offset+6..offset+8]),
            flags: util::from_2_bytes_to_uint(&data[offset+8..offset+10]),
        };
        result.push(thing);

        offset = offset + 10;
    }
    return result;
}

fn resolve_linedefs(data: &Vec<u8>) -> Vec<WadLineDef> {
    let mut result:Vec<WadLineDef> = Vec::new();

    let mut offset = 0;
    while offset < data.len() {
        let new_item = WadLineDef {
            start: util::from_2_bytes_to_uint(&data[offset..offset+2]),
            end: util::from_2_bytes_to_uint(&data[offset+2..offset+4]),
            flags: util::from_2_bytes_to_uint(&data[offset+4..offset+6]),
            special_type: util::from_2_bytes_to_uint(&data[offset+6..offset+8]),
            sector_tag: util::from_2_bytes_to_uint(&data[offset+8..offset+10]),
            right_side_def: util::from_2_bytes_to_uint(&data[offset+10..offset+12]),
            left_side_def: util::from_2_bytes_to_uint(&data[offset+12..offset+14]),
        };
        result.push(new_item);
        offset = offset + 14;
    }

    return result;
}

fn resolve_sidedefs(data: &Vec<u8>) -> Vec<WadSideDef> {
    let mut result:Vec<WadSideDef> = Vec::new();

    let mut offset = 0;
    while offset < data.len() {
        let new_item = WadSideDef {
            x_offset: util::from_2_bytes_to_uint(&data[offset..offset+2]),
            y_offset: util::from_2_bytes_to_uint(&data[offset+2..offset+4]),
            upper_texture: util::from_bytes_to_string(&data[offset+4..offset+12]),
            middle_texture: util::from_bytes_to_string(&data[offset+12..offset+20]),
            lower_texture: util::from_bytes_to_string(&data[offset+20..offset+28]),
            num_faces: util::from_2_bytes_to_uint(&data[offset+28..offset+30]),
        };
        result.push(new_item);
        offset = offset + 30;
    }

    return result;
}

fn resolve_sectors(data: &Vec<u8>) -> Vec<WadSector> {
    let mut result:Vec<WadSector> = Vec::new();
    let mut offset = 0;
    while offset < data.len() {
        let new_item = WadSector {
            floor_height: util::from_2_bytes_to_uint(&data[offset..offset+2]),
            ceiling_height: util::from_2_bytes_to_uint(&data[offset+2..offset+4]),
            floor_tex_name: util::from_bytes_to_string(&data[offset+4..offset+12]),
            ceiling_tex_name: util::from_bytes_to_string(&data[offset+12..offset+20]),
            light_level: util::from_2_bytes_to_uint(&data[offset+20..offset+22]),
            sector_type: util::from_2_bytes_to_uint(&data[offset+22..offset+24]),
            tag: util::from_2_bytes_to_uint(&data[offset+24..offset+26])
        };
        result.push(new_item);
        offset = offset + 26;
    }

    return result;
}

fn resolve_segs(data: &Vec<u8>) -> Vec<WadSeg> {
    let mut result:Vec<WadSeg> = Vec::new();

    let mut offset = 0;
    while offset < data.len() {
        let new_item = WadSeg {
            start: util::from_2_bytes_to_uint(&data[offset..offset+2]),
            end: util::from_2_bytes_to_uint(&data[offset+2..offset+4]),
            angle: util::from_2_bytes_to_uint(&data[offset+4..offset+6]),
            line_def_index: util::from_2_bytes_to_uint(&data[offset+6..offset+8]),
            dir: util::from_2_bytes_to_uint(&data[offset+8..offset+10]),
            offset: util::from_2_bytes_to_uint(&data[offset+10..offset+12]),
        };
        result.push(new_item);
        offset = offset + 12;
    }

    return result;
}


fn resolve_ssectors(data: &Vec<u8>) -> Vec<WadSSector> {
    let mut result:Vec<WadSSector> = Vec::new();

    let mut offset = 0;
    while offset < data.len() {
        let new_item = WadSSector {
            seg_count: util::from_2_bytes_to_uint(&data[offset..offset+2]),
            first_seg_number: util::from_2_bytes_to_uint(&data[offset+2..offset+4]),
        };
        result.push(new_item);
        offset = offset + 4;
    }
    return result;
}


fn resolve_nodes(data: &Vec<u8>) -> Vec<WadNode> {
    let mut result:Vec<WadNode> = Vec::new();

    let mut offset = 0;
    while offset < data.len() {
        let new_item = WadNode {
            line_x: util::from_2_bytes_to_uint(&data[offset..offset+2]),
            line_y: util::from_2_bytes_to_uint(&data[offset+2..offset+4]),
            change_x: util::from_2_bytes_to_uint(&data[offset+4..offset+6]),
            change_y: util::from_2_bytes_to_uint(&data[offset+6..offset+8]),
            r_bbox: from_bytes_to_bbox(&data[offset+8..offset+16]),
            l_bbox: from_bytes_to_bbox(&data[offset+16..offset+24]),
            right_child: util::from_2_bytes_to_uint(&data[offset+24..offset+26]),
            left_child:  util::from_2_bytes_to_uint(&data[offset+26..offset+28]),
        };
        result.push(new_item);
        offset = offset + 28;
    }
    return result;
}

fn from_bytes_to_bbox(arr_u8: &[u8]) -> WadBBox {
    if arr_u8.len() != 8 {
        panic!("byte array was not of length 8");
    }

    return WadBBox {
        top: util::from_2_bytes_to_uint(&arr_u8[0..2]),
        bottom: util::from_2_bytes_to_uint(&arr_u8[2..4]),
        left: util::from_2_bytes_to_uint(&arr_u8[4..6]),
        right: util::from_2_bytes_to_uint(&arr_u8[6..8])
    }
}