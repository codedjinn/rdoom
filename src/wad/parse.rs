use super::{
    FileLump,
    Wad,
    WadHeader,
    WadLump,
    WadLumpType,
    WadType,
};
use super::{util};
use super::{doom1};

use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use anyhow::Result;

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

        let mut marker_index = 0u8;
        for file_lump in file_lumps {

            file.seek(SeekFrom::Start(file_lump.file_pos as u64))?;

            let mut bytes = vec![0; file_lump.size as usize];

            let is_empty = bytes.is_empty().clone();

            file.read_exact(&mut bytes)?;

            // MUST FIX: implemented a really lazy way to link lumps with markers. 
            // It is very faulty especially since E1M9 will mark all lumps after BLOCKMAP as
            // being the part of E1M9
            // Potential fix:
            // Maps have fixed amount of lumps that are linked, just create lookup index and match
            // THINGS, LINEDEFS, SIDEDEFS, VERTEXES, SEGS, SSECTORS, NODES, SECTORS, REJECT, BLOCKMAP

            // super bad...contruction and not using
            let lump_data = if is_empty {
                Vec::new()
            }
            else {
                bytes
            };

             lumps.push(WadLump {
                name: file_lump.name.clone(),
                data: lump_data,
                lump_type: if is_empty { WadLumpType::Marker } else { file_lump.data_type.clone() },
            });
        }

        Ok(lumps)
    }
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
