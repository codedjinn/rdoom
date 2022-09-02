use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(startup);
  }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
   

    // spawning a plane for reference to orient oneself in the world
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // commands.spawn_bundle(Camera3dBundle {
    //     transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
//   commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//   // we will be using ui elements so just add it here
//   commands.spawn_bundle(UiCameraBundle::default());  
}

