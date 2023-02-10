
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

lazy_static! {
    static ref DOOM1_LOOKUP: HashMap<String, WadLumpType> = {
        let mut m = HashMap::new();
        m.insert(String::from("PLAYPAL\0"), WadLumpType::Palette);
        m.insert(String::from("COLORMAP"), WadLumpType::ColorMap);
        m.insert(String::from("SEGS\0\0\0\0"), WadLumpType::Segs);
        m.insert(String::from("THINGS\0\0"), WadLumpType::Things);
        m
    };
}
