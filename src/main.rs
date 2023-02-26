
mod wad;
mod game;

use bevy::{prelude::*, utils::HashMap};
use bevy_flycam::{PlayerPlugin,MovementSettings};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

use anyhow::Result;

use wad::{WadLumpType, WadColor};


fn main() -> Result<()> {

    // read file
    let file = std::fs::File::open("assets/doom1.wad")?;

    // read WAD header
    let wad = wad::Wad::load_from_file(&file)?;

    // parse raw data in usable objects
    let wad_assets = wad::WadAssets::load_from(&wad)?;
    
    App::new()
        .insert_resource(game::GameData { wad_assets: wad_assets })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 100.0, // default: 12.0
        })
        .add_startup_system(setup)
        .add_startup_system(draw_map)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut lines: ResMut<DebugLines>
) {
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 5.0),
    //     ..default()
    // });

    // lines.line_gradient(
    //     Vec3::new(1.0, -1.0, -1.0),
    //     Vec3::new(-1.0, 1.0, 1.0),
    //     9.0,
    //     Color::CYAN,
    //     Color::MIDNIGHT_BLUE,
    // );
}

fn draw_map(
    game_data: Res<game::GameData>,
    mut lines: ResMut<DebugLines>
) {
    let assets = &game_data.wad_assets;
    let vertexes = assets.get_vertexes();
    let line_defs = assets.get_line_defs();

    let count = 0;
    for _ in 0..2 {
        for line_def in line_defs {
            let start_vec = &vertexes[line_def.start as usize];
            let end_vec = &vertexes[line_def.end as usize];

            let sx = start_vec.x as f32;// / 100f32;
            let sy = start_vec.y as f32;// / 100f32;
            let ex = end_vec.x as f32;// / 100f32;
            let ey = end_vec.y as f32;// / 100f32;
            lines.line_gradient(
                Vec3::new(sx, sy, -100.0f32),
                Vec3::new(ex, ey, -100.0f32),
                1000.0,
                Color::RED,
                Color::RED,
            );
        }        
    }
}
