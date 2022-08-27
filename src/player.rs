use bevy::{prelude::*, render::render_resource::Texture};
use bevy_rapier3d::prelude::*;

use std::f32::consts::PI;

#[derive(Component)]
struct Player {
    image_handle: Handle<Image>
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
    }
}


// fn pew(
//     mouse_button_input: Res<Input<MouseButton>>,
//     cameras: Query<(&Transform, &Camera3d)>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut commands: Commands
// ) {
//     if mouse_button_input.just_released(MouseButton::Left) {
//         // for (transform, camera) in cameras.iter() {
//         //     commands
//         //     .spawn()
//         //     .insert(RigidBody::Dynamic)
//         //     .insert(Collider::ball(0.5))
//         //     .insert(Restitution::coefficient(0.7))
//         //     .insert( Velocity {
//         //         linvel: Vec3::new(10.0, 0.0, 0.0),
//         //         angvel: transform.back()
//         //     })
//         //     .insert_bundle(TransformBundle::from(transform.clone()))
//         //     .insert_bundle(PbrBundle {
//         //         mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.5, subdivisions: 16 })),
//         //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//         //         transform: transform.clone(),
//         //         ..default()
//         //     });
//         // }
//     }
// }

// #[derive(Component)]
// struct VoxelModel {

// }