use bevy::prelude::*;
use heron::prelude::*;

use crate::debug::DebugDisplayInfo;

use super::shared;

enum PlayerFacing {
    None,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NnorthWest
}

#[derive(Component)]
pub struct Player {
    pub acc: f32,
    pub vel: f32,
    z: f32,
}

impl Player {
    pub fn new(acc: f32, vel: f32) -> Self {
        Player {
            acc,
            vel,
            z: 0.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_player)
            .add_system(player_movement);
    }
}

fn startup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform {
        translation: Vec3::new(0.0, 0.0, 0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    let sprite = SpriteBundle {
        transform,
        texture: asset_server.load("textures/dude/playb5.png"),
        ..Default::default()
    };

    commands.spawn()
        .insert(Player { 
            acc: 0.0,
            vel: 0.0,
            z: 0.0,
        })
        .insert(CollisionShape::Cuboid
            { 
                half_extends: Vec3::new(32.0, 32.0, 0.0),
                border_radius: Some(0.0)
            })
        .insert_bundle(sprite);
       
}

const move_speed:f32 = 150.0;

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query_debug: Query<&mut DebugDisplayInfo>,
    mut query_player: Query<(&mut Player, &mut Transform)>,
) {
    let mut debug_info = query_debug.single_mut();
    let (mut player, mut transform) = query_player.single_mut();

    let up = keys.pressed(KeyCode::W);
    let down = keys.pressed(KeyCode::S);
    let left = keys.pressed(KeyCode::A);
    let right = keys.pressed(KeyCode::D);

    let k = keys.pressed(KeyCode::K);

    debug_info.output1 = format!("Player Z: {:?}", player.z);
    debug_info.output2 = format!("Transform: {:?}", transform.translation);
    if k {
        player.z += 0.01;
        transform.translation.z = player.z;
    }

    if up {
        transform.translation.y = transform.translation.y + (move_speed * time.delta_seconds());
    }
    else if down {
        transform.translation.y = transform.translation.y - (move_speed * time.delta_seconds());
    }

    if left {
        transform.translation.x = transform.translation.x - (move_speed * time.delta_seconds());
    } else if right {
        transform.translation.x = transform.translation.x + (move_speed * time.delta_seconds());
    }
    // else if down {
    //     transform.translation.y = transform.translation.y - (move_speed * time.delta_seconds());
    // }

    // if left {
    //     transform.translation.x = transform.translation.x - (move_speed * time.delta_seconds());
    // }
    // else if right {
    //     transform.translation.x = transform.translation.x + (move_speed * time.delta_seconds());
    // }

    // debug_info.output1 = format!("{:?}", transform.translation); 

    // let mut dy = 0.0;
    // let mut dir:f32 = 1.0;

    // if up {
    //     player.acc = player.acc + 0.1;
    //     key_pressed = true;
    // }
    // else if down {
    //     player.acc = player.acc + 0.1;
    //     dir = dir * -1.0;
    //     key_pressed = true;
    // }

    // if left {
    //     //body.linvel = Vec2::new(-100.0, 0.0).into();
    //     key_pressed = true;
    // }
    // else if right {
    //     //body.linvel = Vec2::new(100.0, 0.0).into();
    //     key_pressed = true;
    // }

    // debug_info.output1 = format!("acc - {:?}", player.acc);

    // dy = player.acc * move_speed * time.delta_seconds() * dir;
    // transform.translation.y = transform.translation.y + dy;
    
    // if !key_pressed {
    //     player.acc = player.acc - 0.1;
    //     if player.acc < 0.0 {
    //         player.acc = 0.0;
    //     }
    // }
}

