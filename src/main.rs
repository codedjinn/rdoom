mod wad;
mod game;
mod rendering;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_flycam::{
    PlayerPlugin,
    MovementSettings
};
use bevy_prototype_debug_lines::{
    DebugLines,
    DebugLinesPlugin
};
use rendering::{debug, render_map_walls};

use anyhow::Result;

use wad::{WadLevel};
fn main() -> Result<()> {

    // read file
    let file = std::fs::File::open("assets/doom1.wad")?;

    // read WAD header
    let wad = wad::Wad::load_from_file(&file)?;

    // parse raw data in usable objects
    let wad_assets = wad::WadAssets::load_from(&wad)?;

    App::new()
        .insert_resource(game::GameData { 
            wad_assets: wad_assets,
            player_start: Vec3::ZERO,
            start_set: false,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(MovementSettings {
            sensitivity: 0.00012, // default: 0.00012
            speed: 100.0, // default: 12.0
        })
        
        // debug stuff here for now, when it grows move into plugins
        .add_startup_system(debug::debug_anchor)
        .add_startup_system(debug::debug_map_outline_render)
        .add_startup_system(debug::debug_render_things)
        .add_startup_system(extract_game_data)
        .add_startup_system(render_map_walls)
        .add_system(set_player_position)
        .add_system(handle_input)
        .run();

    Ok(())
}

fn handle_input(
    mut game_data: ResMut<game::GameData>,
    mut query: Query<(&Camera3d, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
) {
    // jump between world origin and map player start for debugging
    if keys.just_pressed(KeyCode::O) {
        let mut trans = query.single_mut().1;
        trans.translation = Vec3::ZERO;
    }
    else if keys.just_pressed(KeyCode::P) {
         let mut trans = query.single_mut().1;
        trans.translation = game_data.player_start;
    }
}

fn setup(
) {
   
}

fn extract_game_data(
    mut game_data: ResMut<game::GameData>
) {
    for thing in game_data.wad_assets.get_map(WadLevel::E1M1).get_things() {
        if thing.type_id == 1 {
            game_data.player_start = Vec3::new(thing.x as f32, 0f32, thing.y as f32);
            break;
        }
    }
}

fn set_player_position(
    mut game_data: ResMut<game::GameData>,
    mut query: Query<(&Camera3d, &mut Transform)>,
) {
    if game_data.start_set {
        return;
    }

    if !query.is_empty() {
        println!("NOT EMPTY!");
        let mut trans = query.single_mut().1;
        trans.translation = game_data.player_start;
        game_data.start_set = true;
    }
}
