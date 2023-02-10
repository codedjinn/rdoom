mod util;
mod doom1;

//use std::path::Path;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use anyhow::Result;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum WadType {
    // Eish, it's broh-ken
    Unknown,
    IWAD,
    PWAD,
    WAD2,
}

pub struct Wad {
    header: WadHeader,
    lumps: Vec<WadLump>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WadLumpType {
    Things,
    LineDefs,
    SideDefs,
    Vertexes,
    Segs,
    SSectors,
    Nodes,
    Sectors,
    Reject,
    BlockMap,
    Behavior,
    Marker,

    Palette,
    ColorMap,

    SharewareDoom,

    Unknown
}

#[derive(Debug)]
pub struct WadLump {
    name: String,
    bytes: Vec<u8>,
    lump_type: WadLumpType
}

impl WadLump {

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn lump_type(&self) -> WadLumpType {
        self.lump_type
    }
}

// Interim structure to resolve WadLump
pub struct FileLump {
    file_pos: u32,
    size: u32,
    name: String,
    index: u32,
    data_type: WadLumpType
}

impl Wad {
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
        
        Ok(Wad {
            header,
            lumps
        })
    }

    fn parse_legacy(mut file: &File, num_lumps: u32) -> Result<Vec<FileLump>> {
        let mut result: Vec<FileLump> = Vec::new();

        for i in 0..num_lumps {
            let mut lump_bytes: [u8;16] = [0;16];

            file.read_exact(&mut lump_bytes)?;
    
            let file_pos = util::byte_array_4_to_int(&lump_bytes[0..4]);
            let size = util::byte_array_4_to_int(&lump_bytes[4..8]);
            let name = String::from_utf8(lump_bytes[8..16].to_vec())?;

            let wad_type = doom1::get_wad_type_from_name(&name);

            println!("name {}, wad_type {:?}", name, wad_type);

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
                bytes: temp,
                lump_type: if is_empty { WadLumpType::Marker } else { file_lump.data_type.clone() }
            });
        }

        Ok(lumps)
    }

    pub fn header(&self) -> WadHeader {
        self.header
    }

    pub fn file_lumps(&self) -> &Vec<WadLump> {
        &self.lumps
    }
}

#[derive(Clone, Copy)]
pub struct WadHeader {
    my_type: WadType,
    num_lumps: u32,
    dir_offset: u32,
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

        let num_lumps = util::byte_array_4_to_int(&header_bytes[4..8]);
        let dir_offset = util::byte_array_4_to_int(&header_bytes[8..12]);
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

