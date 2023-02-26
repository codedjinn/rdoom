pub mod util;
mod doom1;
mod parse;
mod picture;

use picture::{WadPicture};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum WadType {
    // Eish, it's broh-ken
    Unknown,
    IWAD,
    PWAD,
    WAD2,
}

pub struct Wad {
   // header: WadHeader,
    lumps: Vec<WadLump>,
}

impl Wad {
    pub fn new(lumps: Vec<WadLump>) -> Self {
        Wad {
            lumps
        }
    }

    // pub fn get_palette(&self, index: usize) -> &WadPalette {
    //     &self.palettes[index]
    // }

    pub fn lumps(&self) -> &Vec<WadLump> {
        &self.lumps
    }
}

#[derive(Debug)]
pub struct WadLump {
    name: String,
    data: Vec<u8>,
    lump_type: WadLumpType
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

// WAD Assets
pub struct WadAssets {
    palettes: Vec<WadPalette>,
    pictures: Vec<WadPicture>,
    things: Vec<WadThing>,
    line_defs: Vec<WadLineDef>,
    vertexes: Vec<WadVertex>,
    sectors: Vec<WadSector>,
}

impl WadAssets {
    pub fn get_palettes(&self) -> &Vec<WadPalette> {
        &self.palettes
    }

    pub fn get_pictures(&self) -> &Vec<WadPicture> {
        &self.pictures
    }

    pub fn get_things(&self) -> &Vec<WadThing> {
        &self.things
    }

    pub fn get_line_defs(&self) -> &Vec<WadLineDef> {
        &self.line_defs
    }

    pub fn get_vertexes(&self) -> &Vec<WadVertex> {
        &self.vertexes
    }

    pub fn get_sectors(&self) -> &Vec<WadSector> {
        &self.sectors
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

pub struct WadThing {
    pub x: u16,
    pub y: u16,
    pub rot: u16,
    pub type_id: u16,
    pub flags: u16
}

// start & end are indexes in WadVertexes
pub struct WadLineDef {
    pub start: u16,
    pub end: u16,
    pub flags: u16,
    pub special_type: u16,
    pub sector_tag: u16,
    pub right_side_def: u16,
    pub left_side_def: u16
}

pub struct WadVertex {
    pub x: u16,
    pub y: u16
}

impl std::fmt::Display for WadVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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