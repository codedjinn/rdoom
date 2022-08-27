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

use bevy::prelude::*;

use bevy_flycam::PlayerPlugin;
use bevy_flycam::MovementSettings;
use game::GamePlugin;

fn main() {

 //   load_wad("doom");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(wad_asset_generator::WadAssetGeneratorPlugin)
        .add_plugin(PlayerPlugin)
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
        .run();
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
