mod wad_asset_generator;
mod game;
mod debug;
mod shared;
mod startup;
mod text;
mod player;
mod doom;

#[macro_use]
extern crate lazy_static;

use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};

use tri_mesh::*;

use bevy_flycam::PlayerPlugin;
use bevy_flycam::MovementSettings;
use game::GamePlugin;
use wad_asset_generator::WadResourceTracker;

struct MyDebugResource {
    parsed_colors: bool,
    color_data: Vec<[u8; 4]>,
    debug_model_spawn: bool
}

impl MyDebugResource {
    fn new() -> Self {
        MyDebugResource {
            parsed_colors: false,
            color_data: Vec::new(),
            debug_model_spawn: false,
        }
    }
}

fn main() {

 //   load_wad("doom");

    App::new()
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)        
        .add_plugin(GamePlugin)
        .add_plugin(wad_asset_generator::WadAssetGeneratorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WireframePlugin)
        .insert_resource(MyDebugResource::new())
        .add_startup_system(set_wrireframe)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0, // default: 12.0
        })
        //.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
      //  .add_plugin(RapierDebugRenderPlugin::default())
      //  .add_plugin(Physics::default())
        // .add_plugin(heron::PhysicsPlugin::default())
        .add_plugin(startup::StartupPlugin)
        .add_plugin(player::PlayerPlugin)
    //    .add_plugin(DemoPlugin)
        // .add_plugin(debug::DebugPlugin)
        //.add_plugin(level::LevelPlugin)
        //.add_plugin(player::PlayerPlugin)
      //  .add_startup_system(setup_physics)
        .add_system(core_keyboard_input)
        .add_system(test_model)
        .run();
}

fn set_wrireframe(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

 //   wireframe_config.global = true;
}

fn test_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_tracker: ResMut<WadResourceTracker>,
    mut debug_resource: ResMut<MyDebugResource>,
    server: Res<Assets<Image>>
) {
    if debug_resource.debug_model_spawn == true {
        return;
    }

    println!("spawning...");

    if asset_tracker.is_loaded && !debug_resource.parsed_colors {
        debug_resource.parsed_colors = true;

        println!("assets parsed!");

        let handle = asset_tracker.debug.as_ref().unwrap();
           
        let img = server.get(&handle).unwrap();
    
        let mut i = 0;
        while i < img.data.len() {
            debug_resource.color_data.push([
                img.data.get(i).unwrap().clone(),
                img.data.get(i + 1).unwrap().clone(),
                img.data.get(i + 2).unwrap().clone(),
                img.data.get(i + 3).unwrap().clone()
            ]);
            i = i + 4;
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    for y in 0..54 {
        for x in 0..35 {

            let index = 35 * y + x;
            let color = debug_resource.color_data.get(index);
            
            if color == None {
                continue;
            }

            let color = color.unwrap();

            if color[3] == 255 {
                let fx = x as f32 * 0.1;
                let fy = y as f32 * 0.1;

                let fr = color[0] as f32 / 255_f32;
                let fg = color[1] as f32 / 255_f32;
                let fb = color[2] as f32 / 255_f32;

                println!("r {} g {} b {}", fr, fg, fb);
                
                println!("fx {} fy {}", fx, fy);
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                    material: materials.add(Color::rgb(fr, fg, fb).into()),
                    transform: Transform::from_xyz(fx, fy, 0.0),
                    ..default()
                });    
            }
        }

        if count > 200 {
            break;
        }
        count = count + 1;
    }

    debug_resource.debug_model_spawn = true;

//     let mut mesh1 = MeshBuilder::new().cube().build().unwrap();
//     mesh1.scale(0.1);
//     let mut mesh2 = MeshBuilder::new().cube().build().unwrap();
//     mesh2.scale(0.1);

//     let pos = tri_mesh::mesh::math::Vec3 { x:0.2, y:0.0, z:0.0};
//     mesh2.translate(pos);

//     // Split the two meshes at their intersection creating two sets of sub meshes
//     let (mut meshes1, mut meshes2) = mesh1.split_at_intersection(&mut mesh2);

//     // Choose two sub meshes to merge (here we just choose one sub mesh from each of the original meshes)
//     let mut result = meshes1.first().unwrap().clone();
//     result.merge_with(meshes2.first().unwrap()).unwrap();

//     let mut merged_mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    
//     let mut vertices: Vec<[f32; 3]> = Vec::new();
//     let mut normals: Vec<[f32; 3]> = Vec::new();
//   //  let mut uvs: Vec<[f32; 2]> = Vec::new();

//     let mut i = 0;
//     while (i < result.positions_buffer_f32().len()) {
//         let x = result.positions_buffer_f32().get(i).unwrap().clone();
//         let y = result.positions_buffer_f32().get(i + 1).unwrap().clone();
//         let z = result.positions_buffer_f32().get(i + 2).unwrap().clone();
//         vertices.push([x,y,z]);
//         i = i + 3;
//     }

//     i = 0;
//     while (i <result.normals_buffer_f32().len()) {
//         let x = result.normals_buffer_f32().get(i).unwrap().clone();
//         let y = result.normals_buffer_f32().get(i + 1).unwrap().clone();
//         let z = result.normals_buffer_f32().get(i + 2).unwrap().clone();
//         normals.push([x,y,z]);
//         i = i + 3;
//     }

//     let merged_indices = bevy::render::mesh::Indices::U32(result.indices_buffer());
//     merged_mesh.set_indices(Some(merged_indices));
//     merged_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
//     merged_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    
//     commands.spawn_bundle(PbrBundle {
//         mesh: meshes.add(merged_mesh),
//         material: materials.add(Color::rgb(1.0, 0.5, 0.3).into()),
//         ..Default::default()
//     });

    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
}

//fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    // commands
    //     .spawn()
    //     .insert(Collider::cuboid(100.0, 0.1, 100.0))
    //         .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0))); 

    /* Create the bouncing ball. */
    // commands
    //     .spawn()
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::ball(0.5))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
//}

// fn load_wad(name: &str) {
//     let cur_dir = std::env::current_dir().expect("Cannot resolve current directory");
//     let cur_dir_as_str = cur_dir.as_os_str().to_str().expect("Couldn't convert OsStr to str");
//     let full_path = format!("{}\\assets\\{name}.wad", cur_dir_as_str);
//     let w = doom::wad::Wad::from_path(full_path);
// }

fn core_keyboard_input(
    input: Res<Input<KeyCode>>,

) {
    if input.pressed(KeyCode::F1) {
        std::process::exit(0);
    }
}

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_scene);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // plane
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });

    // // cube
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });

    // // light
    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });
}
// fn demo_physics(mut commands: Commands) {
//     let collider = ColliderBundle {
//         shape: ColliderShape::ball(40.0).into(),

//         ..Default::default()
//     };
//     commands.spawn_bundle(collider)
//         .insert(ColliderPositionSync::Discrete)
//         .insert(ColliderDebugRender::with_id(1));
// }
