pub mod debug;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_prototype_debug_lines::DebugLines;

use crate::game::{GameData};
use crate::wad::{WadLevel};

#[derive(Component)]
struct WallBlock;

// probably gonna kill system
pub fn render_map_walls(
    game_data: Res<GameData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_wall_texture())),
        ..default()
    });

    let cube = meshes.add(shape::Cube {
        size: 10f32
    }.into());

    let map = game_data.wad_assets.get_map(WadLevel::E1M1);

    let vertexes = map.get_vertexes();

    let lines_defs = map.get_line_defs();

    let mut render_count = 0;

    for line_def in lines_defs {
        
        if render_count > 10 {
            break;
        }

        let start = &vertexes[line_def.start as usize].to_vec();
        let end = &vertexes[line_def.end as usize].to_vec();

        let angle = start.angle_between(end.clone());

        // let mut sx = start.x;
        // let mut sz = start.z;
        // for _ in 0..10 {
        //     //println!("{}", sz * angle);
        //     let dx = sx + (sx * angle);
        //     let dz = sz + (sz * angle);
        //     commands.spawn((
        //         PbrBundle {
        //             mesh: cube.clone(),
        //             material: debug_material.clone(),
        //             transform: Transform::from_xyz(
        //                 dx,
        //                 0.0,
        //                 dz,
        //             )
        //             .with_rotation(Quat::IDENTITY),
        //             ..default()
        //         },
        //         WallBlock,
        //     ));
        //     sx = dx;
        //     sz = dz;
        // }
        render_count = render_count + 1;
    }
}

/// Creates a colorful test pattern
fn uv_wall_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 0, 255, 255,
        255, 159, 102, 255,
        236, 255, 102, 255,
        121, 255, 102, 255,
        102, 255, 198, 255,
        102, 198, 255, 255,
        121, 102, 255, 255,
        236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}