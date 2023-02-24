
use super::doom1::DOOM1_SPRITES;
use super::picture::parse_picture;
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
    Thing
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
        let mut lumps: Vec<WadLump> = Vec::new();

        for file_lump in file_lumps {

            file.seek(SeekFrom::Start(file_lump.file_pos as u64))?;

            let mut bytes = vec![0; file_lump.size as usize];

            file.read_exact(&mut bytes)?;

            let is_empty = bytes.is_empty();
            let temp = if is_empty {
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

    pub fn load_from(wad: &Wad) -> Result<WadAssets> {

        let palettes = WadAssets::resolve_palette(wad)?;
        let things = WadAssets::resolve_things(wad)?;

        let find_sprite = wad.lumps().iter().find(|&x| x.name() == DOOM1_SPRITES[1]);
        if find_sprite.is_some() { 
            
            let sprite = find_sprite.unwrap();

            let data = sprite.data();
            let picture = parse_picture(&data, &palettes[0]);
        }

        return Ok(WadAssets {
            palettes,
            things,
        });
    }

    fn resolve_things(wad: &Wad) -> Result<Vec<Thing>> {
        let things:Vec<_>
            = wad
                .lumps()
                .iter()
                .filter(|&x| x.lump_type() == WadLumpType::Things)
                .collect();


        let mut result:Vec<Thing> = Vec::new();

        if !things.is_empty() {
            for thing in things {
                let data = thing.data();
                let mut count = data.len();
                while count > 0 {
                    let thing = Thing {
                        x: data[0],
                        y: data[1],
                        rot: data[3],
                        type_id: data[4],
                        flags: data[5]
                    };
                    result.push(thing);
                    count = count - 5;
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

    pub fn get_default_pal(&self) -> &WadPalette {
        return &self.palettes[0];
    }
}

