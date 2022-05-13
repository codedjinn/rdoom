use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
  }
}

fn move_camera(
    mut query_camera: Query<(&mut Camera, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut camera, mut camera_transform) = camera_query.single_mut();

    let mut new_pos = camera_transform.translation;
    new_pos.y = new_pos.y + 1;
    camera_transform.translation.y = new_pos;  
}