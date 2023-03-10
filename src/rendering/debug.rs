use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_prototype_debug_lines::DebugLines;

use crate::game::{GameData};
use crate::wad::{WadLevel};

#[derive(Component)]
struct DebugShape;

pub fn debug_anchor(
    mut lines: ResMut<DebugLines>,
) {
    lines.line_colored(Vec3::ZERO * 5f32, Vec3::Y, 1000.0, Color::RED);
    lines.line_colored(Vec3::ZERO * 5f32, Vec3::X, 1000.0, Color::GREEN);
    lines.line_colored(Vec3::ZERO * 5f32, Vec3::Z, 1000.0, Color::BLUE);
}

pub fn debug_map_outline_render(
    game_data: Res<GameData>,
    mut lines: ResMut<DebugLines>,
) {

    let assets = &game_data.wad_assets;
    
    let map = assets.get_map(WadLevel::E1M1);

    let vertexes = map.get_vertexes();

    let mut index = 0;
    let sides = map.get_side_defs();
    let sectors = map.get_sectors();
    let lines_defs = map.get_line_defs();
    for line_def in lines_defs {
        let start_vec = &vertexes[line_def.start as usize];
        let end_vec = &vertexes[line_def.end as usize];

        let sx = start_vec.x as f32;
        let sz = start_vec.y as f32;
        let ex = end_vec.x as f32;
        let ez = end_vec.y as f32;

        if line_def.left_side_def > -1 {
            let side = &sides[line_def.left_side_def as usize];
            let sector = &sectors[side.sector as usize];
            let y1 = sector.floor_height as f32;
            let y2 = sector.ceiling_height as f32;
            lines.line_gradient(
                Vec3::new(sx, y1, sz),
                Vec3::new(ex, y1, ez),
                1000.0,
                Color::BLUE,
                Color::BLUE,
            );

            lines.line_gradient(
                Vec3::new(sx, y2, sz),
                Vec3::new(ex, y2, ez),
                1000.0,
                Color::BLUE,
                Color::BLUE,
            );
        }

        if line_def.right_side_def > -1 {
            let side = &sides[line_def.right_side_def as usize];
            let sector = &sectors[side.sector as usize];
            let y1 = sector.floor_height as f32;
            let y2 = sector.ceiling_height as f32;

            println!("{} {}", y1, y2);
            lines.line_gradient(
                Vec3::new(sx, y1, sz),
                Vec3::new(ex, y1, ez),
                1000.0,
                Color::RED,
                Color::RED,
            );

            lines.line_gradient(
                Vec3::new(sx, y2, sz),
                Vec3::new(ex, y2, ez),
                1000.0,
                Color::RED,
                Color::RED,
            );

            lines.line_gradient(
                Vec3::new(sx, y1, sz),
                Vec3::new(sx, y2, sz),
                1000.0,
                Color::GREEN,
                Color::GREEN,
            );

            // lines.line_gradient(
            //     Vec3::new(ex, y1, ez),
            //     Vec3::new(ex, y2, ez),
            //     1000.0,
            //     Color::PINK,
            //     Color::PINK,
            // );

            // lines.line_gradient(
            //     Vec3::new(sx, y1, sz),
            //     Vec3::new(ex, y2, ez),
            //     1000.0,
            //     Color::RED,
            //     Color::RED,
         //   );
        }
        index = index + 1;
    }
}

pub fn debug_render_things(
    game_data: Res<GameData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = game_data.wad_assets.get_map(WadLevel::E1M1);

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let cube = meshes.add(shape::Cube::default().into());

    let things = map.get_things();
    for thing in things {
        let sx = thing.x as f32;
        let sz = thing.y as f32;

        commands.spawn((
            PbrBundle {
                mesh: cube.clone(),
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    sx,
                    0.0,
                    sz,
                )
                .with_rotation(Quat::IDENTITY),
                ..default()
            },
            DebugShape,
        ));
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
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