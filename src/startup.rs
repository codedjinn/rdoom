use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(startup);
  }
}

fn startup(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  // we will be using ui elements so just add it here
  commands.spawn_bundle(UiCameraBundle::default());  
}

