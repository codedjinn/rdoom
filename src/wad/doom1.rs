
// DOOM1 does not have type info for lumps, built this dictionary to match names with types.
// Used Slade editor to get this list

use crate::wad::WadLumpType;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn get_wad_type_from_name(name: &str) -> WadLumpType {
    let wad_type = DOOM1_LOOKUP.get(name);

    if wad_type.is_some() {
        return wad_type.unwrap().clone();
    }
    
    return WadLumpType::Unknown;
}

pub static DOOM1_SPRITES: [&str; 2] = [
    "SARGA1", "TROOE1"
];

// pub static DOOM1_LOOKUP: [&str; 107] = [
//     "FLOOR0_1", "FLOOR0_3", "FLOOR0_6", "FLOOR1_1", "FLOOR1_7", "FLOOR3_3", "FLOOR4_1", "FLOOR4_5",
//     "FLOOR4_6", "FLOOR4_8", "FLOOR5_1", "FLOOR5_2", "FLOOR5_3", "FLOOR5_4", "STEP1", "STEP2",
//     "FLOOR6_1", "FLOOR6_2", "TLITE6_1", "TLITE6_4", "TLITE6_5", "TLITE6_6", "FLOOR7_1", "FLOOR7_2",
//     "MFLR8_1", "DEM1_1", "DEM1_2", "DEM1_3", "DEM1_4", "CEIL3_1", "CEIL3_2", "CEIL3_5", "CEIL4_2",
//     "CEIL4_3", "CEIL5_1", "CEIL5_2", "FLAT1", "FLAT2", "FLAT5", "FLAT10", "FLAT14", "FLAT18",
//     "FLAT20", "FLAT22", "FLAT23", "FLAT5_4", "FLAT5_5", "CONS1_1", "CONS1_5", "CONS1_7", "NUKAGE1",
//     "NUKAGE2", "NUKAGE3", "F_SKY1", "SFLR6_1", "SFLR6_4", "SFLR7_1", "SFLR7_4", "FLOOR0_2",
//     "FLOOR0_5", "FLOOR0_7", "FLOOR1_6", "GATE1", "GATE2", "GATE3", "GATE4", "FWATER1", "FWATER2",
//     "FWATER3", "FWATER4", "LAVA1", "LAVA2", "LAVA3", "LAVA4", "DEM1_5", "DEM1_6", "MFLR8_2",
//     "MFLR8_3", "MFLR8_4", "CEIL1_1", "CEIL1_2", "CEIL1_3", "CEIL3_3", "CEIL3_4", "CEIL3_6",
//     "CEIL4_1", "BLOOD1", "BLOOD2", "BLOOD3", "FLAT1_1", "FLAT1_2", "FLAT1_3", "FLAT5_1", "FLAT5_2",
//     "FLAT5_3", "FLAT5_6", "FLAT5_7", "FLAT5_8", "CRATOP1", "CRATOP2", "FLAT3", "FLAT4", "FLAT8",
//     "FLAT9", "FLAT17", "FLAT19", "COMP01",
// ];

lazy_static! {
    static ref DOOM1_LOOKUP: HashMap<String, WadLumpType> = {
        let mut m = HashMap::new();
        m.insert(String::from("PLAYPAL"), WadLumpType::Palette);
        m.insert(String::from("COLORMAP"), WadLumpType::ColorMap);
        m.insert(String::from("SEGS"), WadLumpType::Segs);
        m.insert(String::from("THINGS"), WadLumpType::Things);
        m.insert(String::from("LINEDEFS"), WadLumpType::LineDefs);
        m.insert(String::from("SIDEDEFS"), WadLumpType::SideDefs);
        m.insert(String::from("VERTEXES"), WadLumpType::Vertexes);
        m.insert(String::from("SEGS"), WadLumpType::Segs);
        m.insert(String::from("SECTORS"), WadLumpType::Sectors);
        m.insert(String::from("SSECTORS"), WadLumpType::SSectors);
        m.insert(String::from("NODES"), WadLumpType::Nodes);
        m.insert(String::from("BLOCKMAP"), WadLumpType::BlockMap);
        m.insert(String::from("F_START"), WadLumpType::FStart);
        m.insert(String::from("F_END"), WadLumpType::FEnd);
        m.insert(String::from("S_START"), WadLumpType::SStart);
        m.insert(String::from("S_END"), WadLumpType::SEnd);
        m
    };
}


// lazy_static! {
//     static ref DOOM1_SPRITES: HashMap<String, String> = {
//         let mut m = HashMap::new();
//         m.insert(String::from("PLAYPAL\0"), String::from(""));
//         m
//     };
// }

