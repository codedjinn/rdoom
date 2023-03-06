
mod wad;
mod game;

use bevy::{prelude::*};
use bevy_flycam::{PlayerPlugin,MovementSettings};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

use anyhow::Result;

use wad::{WadLumpType, WadColor, WadLevel};


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
            sensitivity: 0.00012, // default: 0.00012
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
    // reference point
    lines.line_colored(Vec3::ZERO, Vec3::Y, 1000.0, Color::RED);
    lines.line_colored(Vec3::ZERO, Vec3::X, 1000.0, Color::GREEN);
    lines.line_colored(Vec3::ZERO, Vec3::Z, 1000.0, Color::BLUE);


    let assets = &game_data.wad_assets;
    
    let map = assets.get_map(WadLevel::E1M1);

    let vertexes = map.get_vertexes();

    for i in 0..vertexes.len() {
        println!("x {} y {}", &vertexes[i].x, &vertexes[i].y);
    }

    let lines_defs = map.get_line_defs();
    for line_def in lines_defs {
        let start_vec = &vertexes[line_def.start as usize];
        let end_vec = &vertexes[line_def.end as usize];

        let sx = start_vec.x as f32 / 100f32;
        let sz = start_vec.y as f32 / 100f32;
        let ex = end_vec.x as f32 / 100f32;
        let ez = end_vec.y as f32 / 100f32;
        lines.line_gradient(
            Vec3::new(sx, 0f32, sz),
            Vec3::new(ex, 0f32, ez),
            1000.0,
            Color::RED,
            Color::BLUE,
        );
    }    
    
    // let map = assets.get_maps().iter().find(|&m| m.level == WadMapLevel::E1M1);

    // if map.is_none() {
    //     return;
    // }

    // let vertexes = assets.get_map_vertexes(map.unwrap().index);

    // let verts = &vertexes.verts;

    // let count = 0;
    // for _ in 0..2 {
    //     for line_def in line_defs {
    //         let start_vec = &vertexes[line_def.start as usize];
    //         let end_vec = &vertexes[line_def.end as usize];

    //         let sx = start_vec.x as f32;// / 100f32;
    //         let sy = start_vec.y as f32;// / 100f32;
    //         let ex = end_vec.x as f32;// / 100f32;
    //         let ey = end_vec.y as f32;// / 100f32;
    //         lines.line_gradient(
    //             Vec3::new(sx, sy, -100.0f32),
    //             Vec3::new(ex, ey, -100.0f32),
    //             1000.0,
    //             Color::RED,
    //             Color::RED,
    //         );
    //     }        
    // }
}
