pub mod util;
mod doom1;
mod parse;
mod parse_assets;
mod picture;

use picture::{WadPicture};

use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum WadType {
    // Eish, it's broh-ken
    Unknown,
    IWAD,
    PWAD,
    WAD2,
}

pub struct Wad {
    lumps: Vec<WadLump>,
}

impl Wad {
    pub fn new(lumps: Vec<WadLump>) -> Self {
        Wad {
            lumps
        }
    }

    pub fn lumps(&self) -> &Vec<WadLump> {
        &self.lumps
    }

    pub fn get_lump(&self, index: usize) -> &WadLump {
        return &self.lumps[index];
    }
}

#[derive(Debug)]
pub struct WadLump {
    name: String,    
    lump_type: WadLumpType,
    data: Vec<u8>,
}

impl WadLump {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
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

#[derive(Clone, Copy)]
pub struct WadHeader {
    my_type: WadType,
    num_lumps: u32,
    dir_offset: u32,
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

    FStart,
    FEnd,

    SStart,
    SEnd,

    SharewareDoom,

    Unknown
}


// Hard-coded goodness in support of Doom1 shareware WAD
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WadLevel {
    E1M1,
    E1M2,
    E1M3,
    E1M4,
    E1M5,
    E1M6,
    E1M7,
    E1M8,
    E1M9
}

impl FromStr for WadLevel {
    
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "E1M1" => Ok(WadLevel::E1M1),
            "E1M2" => Ok(WadLevel::E1M2),
            "E1M3" => Ok(WadLevel::E1M3),
            "E1M4" => Ok(WadLevel::E1M4),
            "E1M5" => Ok(WadLevel::E1M5),
            "E1M6" => Ok(WadLevel::E1M6),
            "E1M7" => Ok(WadLevel::E1M7),
            "E1M8" => Ok(WadLevel::E1M8),
            "E1M9" => Ok(WadLevel::E1M9),
            _ => Err(()),
        }
    }
}

// WAD Assets
pub struct WadAssets {
    palettes: Option<Vec<WadPalette>>,
    pictures: Option<Vec<WadPicture>>,
    maps: Option<Vec<WadMap>>
}

impl WadAssets {
    pub fn new() -> Self {
        WadAssets { 
            palettes: None,
            pictures: None,
            maps: None
        }
    }

    pub fn set_palettes(&mut self, pals: Vec<WadPalette>) {
        self.palettes = Some(pals);
    }

    pub fn get_palettes(&self, index: usize) -> &WadPalette {
        &self.palettes.as_ref().unwrap()[index]
    }

    pub fn get_map(&self, level: WadLevel) -> &WadMap {
        let maps = &self.maps.as_ref().unwrap();
        for i in 0..maps.len() {
            let map = &maps[i];
            if map.level == level {
                return map;
            }
        }
        panic!("No level found {:?}", level);
    }
}

// bevy has its own Color struct which utilizes floats,
// this keeps it so that we don't have type conflicts
// Doom data 255 = transparent
#[derive(Clone, Copy)]
pub struct WadColor {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl WadColor {
    pub fn transparent() -> Self {
        return WadColor {
            r: 255, g:255, b:255
        };
    }

    pub fn is_transparent(&self) -> bool {
        return self.r == 255 && self.g == 255 && self.b == 255;
    }
}

// For now will only support Doom1 shareware WAD, so hardcoding sturcture a bit
pub struct WadMap {
    level: WadLevel,

    things: Vec<WadThing>,
    line_defs: Vec<WadLineDef>,
    side_defs: Vec<WadSideDef>,
    vertexes: Vec<WadVertex>,
    segs: Vec<WadSeg>,
    ssectors: Vec<WadSSector>,
    nodes: Vec<WadNode>,
    sectors: Vec<WadSector>,
}

impl WadMap {
    pub fn get_things(&self) -> &Vec<WadThing> {
        &self.things
    }

    pub fn get_line_defs(&self) -> &Vec<WadLineDef> {
        &self.line_defs
    }

    pub fn get_side_defs(&self) -> &Vec<WadSideDef> {
        &self.side_defs
    }

    pub fn get_vertexes(&self) -> &Vec<WadVertex> {
        &self.vertexes
    }

    pub fn get_segs(&self) -> &Vec<WadSeg> {
        &self.segs
    }

    pub fn get_nodes(&self) -> &Vec<WadNode> {
        &self.nodes
    }

    pub fn get_sectors(&self) -> &Vec<WadSector> {
        &self.sectors
    }
}

pub struct WadSeg {
    pub start: u16,
    pub end: u16,
    pub angle: u16,
    pub line_def_index: u16,
    pub dir: u16,
    pub offset: u16,
}

pub struct WadThing {
    pub x: u16,
    pub y: u16,
    pub rot: u16,
    pub type_id: u16,
    pub flags: u16
}

// start & end are indexes in WadVertex
pub struct WadLineDef {
    pub start: u16,
    pub end: u16,
    pub flags: u16,
    pub special_type: u16,
    pub sector_tag: u16,
    pub right_side_def: u16,
    pub left_side_def: u16,
}

pub struct WadSideDef {
    pub x_offset: u16,
    pub y_offset: u16,
    pub upper_texture: String,
    pub middle_texture: String,
    pub lower_texture: String,
    pub num_faces: u16,
}

pub struct WadSector {
    pub floor_height: u16,
    pub ceiling_height: u16,
    pub floor_tex_name: String,
    pub ceiling_tex_name: String,
    pub light_level: u16,
    pub sector_type: u16,
    pub tag: u16
}

pub struct WadSSector {
    pub seg_count: u16,
    pub first_seg_number: u16,
}

pub struct WadNode {
    pub line_x: u16,
    pub line_y: u16,
    pub change_x: u16,
    pub change_y: u16,
    pub r_bbox: WadBBox,
    pub l_bbox: WadBBox,
    pub right_child: u16,
    pub left_child: u16,
}

pub struct WadBBox {
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}

#[derive(Debug)]
pub struct WadVertex {
    pub x: u16,
    pub y: u16,
}

impl std::fmt::Display for WadVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct WadPalette {
    colors: Vec<WadColor>
}

impl WadPalette {
    pub fn new() -> Self {
        WadPalette {
            colors: Vec::new()
        }
    }

    pub fn add(&mut self, color: WadColor) {
        self.colors.push(color);
    }

    pub fn colors(&self) -> &Vec<WadColor> {
        &self.colors
    }
}
