use bevy::prelude::*;

use super::doom::wad::{
    Wad,
    Lump
};

use std::sync::{Arc, atomic::AtomicBool};

//
// Constants
//
pub const DOOM1_PLAYPAL:&str = "PLAYPAL";
pub const DOOM1_COLORMAP:&str = "COLORMAP";

pub struct WadResources {
    wad: Option<Box<Wad>>,
    pub items: Vec<WadResourceItem>,
}

pub struct WadResourceItem {
    is_loaded: bool,
    lump_name: String,
    data: Option<Vec<u8>>,
}

impl WadResourceItem {
    fn new(name: &str) -> Self {
        WadResourceItem {
            is_loaded: false,
            lump_name: String::from(name),
            data: None
        }
    }
}

impl WadResources {
    fn new() -> Self {
        WadResources {
            wad: None,
            items: Vec::new()
        }
    }

    pub fn load(&mut self, name: &str) {
        self.items.push(WadResourceItem::new(name));
    }

    pub fn get(&self, name: &str) -> Option<&WadResourceItem> {
        let n1 = name.to_lowercase();
        let result = self.items.iter()
                         .find(|x| x.lump_name.to_lowercase() == name);
        return result;
    }
}

pub struct WadResourcesPlugin;

impl Plugin for WadResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WadResources::new())
           .add_startup_system(load_wad)
           .add_system(load_resources);
    }
}

//
// Systems
//
fn load_wad(
    server: Res<AssetServer>,
    mut wad_resources: ResMut<WadResources>
) {
    // in future load fron config.json
    let wad = load_wad_file("doom1.wad");

    // make this cancelable
    wad_resources.wad = Some(Box::new(wad));

    wad_resources.load("playa1");
}

fn load_resources(
    server: Res<AssetServer>,
    mut wad_resources: ResMut<WadResources>
) {
    // Exit app if this is true?
    if wad_resources.wad.is_none() {
        return;
    }

    let local_wad_ref = wad_resources.wad
                                .as_deref()
                                .expect("Error reading WAD file");

    if wad_resources.items.len() > 0 {
        for item in wad_resources.items.iter() {
            let entry = local_wad_ref
                                    .get_by_name(&item.lump_name)
                                    .unwrap();

            println!("data type - {:?}", entry.lump().data().data_type());
        }
    }

    // // process 10 at once for now
    // if wad_resources.lumps_to_load.len() > 10 {
    //     for i in 0..9 {
    //         let lump_name = wad_resources.lumps_to_load[i].clone();
    //         let lumpy = get_lump(&local_wad_ref, &lump_name);
    //     }
    // }
    // } else {
    //     while wad_resources.lumps_to_load.len() > 0 {
    //         let mut lump_name = wad_resources.lumps_to_load.pop();
    //     }
    //     for i in 0..wad_resources.lumps_to_load.len() {
    //         let lump_name = wad_resources.lumps_to_load[i].clone();
    //         let lumpy = get_lump(&local_wad_ref, &lump_name);
    //         println!("LUMPY {}", lumpy.is_some());
    //     }
    // }
}

//
// Helpers
//
fn load_wad_file(name: &str) -> Wad {
    let cur_dir = std::env::current_dir().expect("Cannot resolve current directory");
    let cur_dir_as_str = cur_dir.as_os_str().to_str().expect("Couldn't convert OsStr to str");

    println!("CUR DIR {:?}", cur_dir_as_str);
    
    let full_path = format!("{}\\assets\\{name}", cur_dir_as_str);
    
    return Wad::from_path(full_path);
}

fn get_lump<'a>(wad: &'a Wad, name: &'a str) -> Option<&'a Lump> {
    for lump in wad.lumps() {
        if lump.name() == name {
            return Some(lump);
        }
    }
    return None;
}