
//use super::doom1::DOOM1_SPRITES;
//use super::picture::parse_picture;
use super::{
    FileLump,
    Wad,
    WadHeader,
    WadLump,
    WadLumpType,
    WadType,
    WadColor,
    WadPalette,
    WadAssets,
    WadThing,
    WadLineDef,
    WadVertex,
    WadSector
};
use super::{util};
use super::{doom1};
use super::{picture};

use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use anyhow::Result;
use bevy::prelude::Image;

impl Wad {

    // Parses into basic Wad format
    //
    // - Retrieve Wad Header
    // - Parse lumps
    pub fn load_from_file(mut file: &File) -> Result<Wad> {        
        let header = WadHeader::load_from_file(file);

        println!("wad_type {:?}", header.wad_type());

        file.seek(SeekFrom::Start(header.dir_offset() as u64))?;
       
        let mut file_lumps: Option<Vec<FileLump>> = Option::None;
        if header.wad_type() == WadType::IWAD || header.wad_type() == WadType::PWAD {
            file_lumps = Some(Wad::parse_legacy(&file, header.num_lumps())?);
        }
        else {
            // TODO:
            file_lumps = Some(Vec::new());
        }

        let lumps = Wad::parse_lumps(&file, &file_lumps.unwrap())?;

        Ok(Wad::new(lumps))
    }
  
    // For now I only know that Doom1 uses this 'legacy' WAD structure
    fn parse_legacy(mut file: &File, num_lumps: u32) -> Result<Vec<FileLump>> {
        let mut result: Vec<FileLump> = Vec::new();

        let mut count = num_lumps;
        for i in 0..count {
            let mut lump_bytes: [u8;16] = [0;16];

            file.read_exact(&mut lump_bytes)?;
    
            let file_pos = util::from_4_bytes_to_int(&lump_bytes[0..4]);
            let size = util::from_4_bytes_to_int(&lump_bytes[4..8]);
            let name = util::from_bytes_to_string(&lump_bytes[8..16]);

            let wad_type = doom1::get_wad_type_from_name(&name);
            //     file.read_exact(&mut lump_bytes)?;
    
            //     let file_pos2 = util::from_4_bytes_to_int(&lump_bytes[0..4]);
            //     let size2 = util::from_4_bytes_to_int(&lump_bytes[4..8]);
            //     let name2 = String::from_utf8(lump_bytes[8..16].to_vec())?;
            //     println!("name2 {}", name2);
            //     count = count - 1;
            // }
            
            result.push(FileLump {
                file_pos,
                size,
                name,
                index: i,
                data_type: wad_type
            });
        }
        
        Ok(result)
    }

    fn parse_lumps(mut file: &File, file_lumps: &Vec<FileLump>) -> Result<Vec<WadLump>> {        
        
        let mut found_marker = false;
        
        let mut lumps: Vec<WadLump> = Vec::new();

        for file_lump in file_lumps {

            file.seek(SeekFrom::Start(file_lump.file_pos as u64))?;

            let mut bytes = vec![0; file_lump.size as usize];

            file.read_exact(&mut bytes)?;

            // TODO: lazy code :P change to Option later
            let is_empty = bytes.is_empty();
            let temp = if !is_empty {
                Vec::new()
            } else {
                bytes
            };

            lumps.push(WadLump {
                name: file_lump.name.clone(),
                data: temp,
                lump_type: if is_empty { WadLumpType::Marker } else { file_lump.data_type.clone() }
            });
        }

        Ok(lumps)
    }

    //
    // Accessors
    //
   
}

impl WadHeader {    
    fn load_from_file(mut file: &File) -> WadHeader {
        let mut header_bytes: [u8; 12] = [0; 12];
        file.read_exact(&mut header_bytes).expect("Could not parse header of WAD");
        
        let type_bytes = &header_bytes[0..4];
        let mut my_type = WadType::Unknown;
        if b"IWAD" == type_bytes {
            my_type = WadType::IWAD;
        }
        else if b"WAD2" == type_bytes {
            my_type = WadType::WAD2;
        }
        else if b"PWAD" == type_bytes {
            my_type = WadType::PWAD;
        }

        let num_lumps = util::from_4_bytes_to_int(&header_bytes[4..8]);
        let dir_offset = util::from_4_bytes_to_int(&header_bytes[8..12]);
        WadHeader {
            my_type,
            num_lumps,
            dir_offset
        }
    }

    pub fn num_lumps(&self) -> u32 {
        self.num_lumps
    }

    pub fn dir_offset(&self) -> u32 {
        self.dir_offset
    }

    pub fn wad_type(&self) -> WadType {
        self.my_type
    }
}


impl WadAssets {

    pub fn get_default_pal(&self) -> &WadPalette {
        return &self.palettes[0];
    }

    pub fn load_from(wad: &Wad) -> Result<WadAssets> {

        let palettes = WadAssets::resolve_palette(wad)?;
        let things = WadAssets::resolve_things(wad)?;
        let pictures = picture::resolve_pictures(wad, &palettes[0])?;
        let line_defs = WadAssets::resolve_linedefs(wad)?;
        let vertexes = WadAssets::resolve_vertexes(wad)?;
        let sectorsResult = WadAssets::resolve_sectors(wad);
        if sectorsResult.is_err() {
            panic!("WTF");
        }
        let sectors = sectorsResult.ok().unwrap();

        return Ok(WadAssets {
            palettes,
            pictures,
            things,
            line_defs,
            vertexes,
            sectors
        });
    }
    
    fn resolve_vertexes(wad: &Wad) -> Result<Vec<WadVertex>> {
        let vertexes:Vec<_>
            = wad
                .lumps()
                .iter()
                .filter(|&x| x.lump_type() == WadLumpType::Vertexes)
                .collect();

        let mut result:Vec<WadVertex> = Vec::new();
        if !vertexes.is_empty() {
            for vert in vertexes {
                let data = vert.data();
                let mut offset = 0;
                while offset < data.len() {
                    let vertex = WadVertex {
                        x: util::from_2_bytes_to_int(&data[offset..offset+2]),
                        y: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
                    };
                    result.push(vertex);
                    offset = offset + 4;
                }
            }
        }
        return Ok(result);
    }

    fn resolve_sectors(wad: &Wad) -> Result<Vec<WadSector>> {
        let line_defs:Vec<_>
            = wad
                .lumps()
                .iter()
                .filter(|&x| x.lump_type() == WadLumpType::Sectors)
                .collect();

        let mut result:Vec<WadSector> = Vec::new();

        if !line_defs.is_empty() {
            for line_def in line_defs {
                let data = line_def.data();
                let mut offset = 0;
                while offset < data.len() {
                    let new_item = WadSector {
                        floor_height: util::from_2_bytes_to_int(&data[offset..offset+2]),
                        ceiling_height: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
                        floor_tex_name: util::from_bytes_to_string(&data[offset+4..offset+12]),
                        ceiling_tex_name: util::from_bytes_to_string(&data[offset+12..offset+20]),
                        light_level: util::from_2_bytes_to_int(&data[offset+20..offset+22]),
                        sector_type: util::from_2_bytes_to_int(&data[offset+22..offset+24]),
                        tag: util::from_2_bytes_to_int(&data[offset+24..offset+26])
                    };
                    result.push(new_item);
                    offset = offset + 26;
                }
            }
        }
        return Ok(result);
    }


    // fn resolve<T, F: Fn(&[u8]) -> usize>(wad: &Wad, lump_type: WadLumpType, func: F) -> Result<Vec<T>> {

    //     let data = [0u8; 0];

    //     let result = func(&data[0..1]);
    //     println!("{}", result);
    //     return Ok(Vec::new());
    // }

    fn resolve_linedefs(wad: &Wad) -> Result<Vec<WadLineDef>> {
        let line_defs:Vec<_>
            = wad
                .lumps()
                .iter()
                .filter(|&x| x.lump_type() == WadLumpType::LineDefs)
                .collect();

        let mut result:Vec<WadLineDef> = Vec::new();

        if !line_defs.is_empty() {
            for line_def in line_defs {
                let data = line_def.data();
                let mut offset = 0;
                while offset < data.len() {
                    let new_item = WadLineDef {
                        start: util::from_2_bytes_to_int(&data[offset..offset+2]),
                        end: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
                        flags: util::from_2_bytes_to_int(&data[offset+4..offset+6]),
                        special_type: util::from_2_bytes_to_int(&data[offset+6..offset+8]),
                        sector_tag: util::from_2_bytes_to_int(&data[offset+8..offset+10]),
                        right_side_def: util::from_2_bytes_to_int(&data[offset+10..offset+12]),
                        left_side_def: util::from_2_bytes_to_int(&data[offset+12..offset+14])
                    };
                    result.push(new_item);
                    offset = offset + 14;
                }
            }
        }
        return Ok(result);
    }

    fn resolve_things(wad: &Wad) -> Result<Vec<WadThing>> {
        let things:Vec<_>
            = wad
                .lumps()
                .iter()
                .filter(|&x| x.lump_type() == WadLumpType::Things)
                .collect();

        let mut result:Vec<WadThing> = Vec::new();

        if !things.is_empty() {
            for thing in things {
                let data = thing.data();
                //let mut count = data.len();
                let mut offset = 0;
                while offset < data.len() {
                    let thing = WadThing {
                        x: util::from_2_bytes_to_int(&data[offset..offset+2]),
                        y: util::from_2_bytes_to_int(&data[offset+2..offset+4]),
                        rot: util::from_2_bytes_to_int(&data[offset+4..offset+6]),
                        type_id: util::from_2_bytes_to_int(&data[offset+6..offset+8]),
                        flags: util::from_2_bytes_to_int(&data[offset+8..offset+10]),
                    };
                    result.push(thing);

                    offset = offset + 10;
                  //  count = count - 10;
                }

            }
        }
        return Ok(result);
    }

    fn resolve_palette(wad: &Wad) -> Result<Vec<WadPalette>> {
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
}

