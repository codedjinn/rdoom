use bevy::prelude::*;

use crate::game::{GameState, DoomGame};
//use crate::doom::wad::{Wad};

pub struct WadResourceTracker {
    pub is_loaded: bool,
    pub failures: u32,
    
    images: Vec<Handle<Image>>,


    debug: Option<Handle<Image>>,

    once: bool,
}

impl WadResourceTracker {
    fn new() -> Self {
        WadResourceTracker { 
            is_loaded: false, 
            failures: 0, 
            images: Vec::new(),
            debug: None,
            once: true,
        }
    }

    fn get_asset_count(&self) -> u32 {
        self.images.len() as u32
    }
}

pub struct WadAssetGeneratorPlugin;

impl Plugin for WadAssetGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WadResourceTracker::new())
           .add_startup_system(setup)
           .add_system(check_assets)
           .add_system(do_something);
    }
}

pub struct VoxelSpriteData {
    
}

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut doom_game: ResMut<DoomGame>,
    mut asset_tracker: ResMut<WadResourceTracker>
) {
    doom_game.set_state(GameState::GeneratingAssets);
    
    let image: Handle<Image> = server.load("textures/dude/playb5.png");

    asset_tracker.debug = Some(image.clone());

    asset_tracker.images.push(image);
}

fn check_assets(
    server: Res<AssetServer>,
    mut asset_tracker: ResMut<WadResourceTracker>
) {
    if !asset_tracker.is_loaded {
        let mut assets_loaded = 0;
        let mut assets_failed: u32 = 0;
        for handle in &asset_tracker.images {
            match server.get_load_state(handle) {
                bevy::asset::LoadState::Loaded => { assets_loaded = assets_loaded + 1 },
                bevy::asset::LoadState::Failed => { assets_failed = assets_failed + 1 },
                _ => { }
            }
        }
        
        asset_tracker.is_loaded = assets_loaded == asset_tracker.get_asset_count();
        asset_tracker.failures = assets_failed;
    }
}

fn do_something(
    mut asset_tracker: ResMut<WadResourceTracker>,
    server: Res<Assets<Image>>
) {
    if asset_tracker.is_loaded && asset_tracker.once {        
        asset_tracker.once = false;

        let handle = asset_tracker.debug.as_ref().unwrap();
        
        let img = server.get(&handle).unwrap();
        
        //let mut x = 0u32;
        //let mut y = 0u32;
        for x in 0u32..32u32 {
            for y in 0u32..63u32 {
                let pixel = get_pixel(x, y, 63, &img.data);
            }
        }
    }
}

fn get_pixel(x: u32, y: u32, width: u32, data: &Vec<u8>) -> u8 {
    let index = (width * y + x) as usize;
    return data[index];
}


// fn load_wad(name: &str) {
//     let cur_dir = std::env::current_dir().expect("Cannot resolve current directory");
//     let cur_dir_as_str = cur_dir.as_os_str().to_str().expect("Couldn't convert OsStr to str");
//     let full_path = format!("{}\\assets\\{name}.wad", cur_dir_as_str);
//     let w = Wad::from_path(full_path);
// }