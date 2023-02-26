
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

    let wad_assets = wad::WadAssets::load_from(&wad)?;
    
    let lines = wad_assets.get_line_defs();
    let vertices = wad_assets.get_vertexes();

    // App::new()
    //     .insert_resource(game::GameData { wad_assets: wad_assets })
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(PlayerPlugin)
    //     .add_plugin(DebugLinesPlugin::default())
    //     .insert_resource(MovementSettings {
    //         sensitivity: 0.00015, // default: 0.00012
    //         speed: 30.0, // default: 12.0
    //     })
    //     .add_startup_system(setup)
    //     .add_startup_system(draw_map)
    //     .run();


    // let pal = wad_assets.get_default_pal();

//    println!("default pal size {}", pal.colors().len());

   // let pal = wad_result.get_palette(0);

//    let pal = wad_result.get_color_palette(0);

    //println!("PLAYPAL length {}", pal.colors.len());
    // let pal 
    //     = wad_result
    //         .lumps()
    //         .iter()
    //         .find(|&x| x.lump_type() == WadLumpType::Palette);

    // let bytes = pal.unwrap().data();

    //let slice = &bytes[0..3];
    //let r = wad::util::byte_array_4_to_int(slice);
        
    // let mut new_image = bmp::Image::new(255, 1);

    // let mut i = 0usize;
  

    // new_image.save("d:\\pal2.bmp");

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

    let sectors = assets.get_sectors();

    let mut dic = Dic::new();

    let count = 0;
    for _ in 0..1 {
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


struct KeyItem {
    key: u16,
    values: Vec<u16>
}

impl KeyItem {
    fn new(key: u16) -> Self {
        KeyItem {
            key,
            values: Vec::new()
        }
    }

    fn add(&mut self, item: u16) {
        self.values.push(item);
    }
}

struct Dic {
    entries: Vec<KeyItem>
}

impl Dic {

    fn print(&self) {

        for entry in &self.entries {
            print!("key {}", entry.key);
            for value in &entry.values {
                print!(" {}", value);
            }
            println!("");
        }

    }

    fn new() -> Self {
        Dic {
            entries: Vec::new()
        }
    }

    fn set(&mut self, key: u16, value: u16) {

        let mut index:Option<usize> = None;
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.key == key {
                index = Some(i);
                break;
            }
        }
        
        if index.is_none() {
            let mut key_item = KeyItem::new(key);
            key_item.add(value);
            self.entries.push(key_item);
        }
        else {
            self.entries[index.unwrap()].add(value);
        }
    }

}