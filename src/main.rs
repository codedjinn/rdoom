
mod text;
mod startup;
mod debug;
mod player;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierRenderPlugin)
    .add_plugin(startup::StartupPlugin)
    .add_plugin(DemoPlugin)
    .add_plugin(debug::DebugPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_system(core_keyboard_input)
    .run();
}

fn core_keyboard_input(input: Res<Input<KeyCode>>) {
  if input.pressed(KeyCode::F1) {
    std::process::exit(0);
  }
}

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
  fn build(&self, app: &mut App) {
      app.add_startup_system(demo_physics);
  }
}

fn demo_physics(mut commands: Commands) {
    let collider = ColliderBundle {
        shape: ColliderShape::ball(2.0).into(),
        ..Default::default()
    };
    commands.spawn_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1));
}
