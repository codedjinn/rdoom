use bevy::prelude::*;

use crate::debug::DebugDisplayInfo;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_player)
            .add_system(player_input);
    }
}

fn startup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = SpriteBundle {
        texture: asset_server.load("textures/dude/playb5.png"),
        ..Default::default()
    };

    commands.spawn()
        .insert(Player)
        .insert_bundle(sprite);
}

fn player_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query_debug: Query<&mut DebugDisplayInfo>,
    mut query_player: Query<&mut Transform, With<Player>>,
) {
    let mut debug_info = query_debug.single_mut();
    let mut player_transform = query_player.single_mut();

    if keys.pressed(KeyCode::W) {
        let new_pos = player_transform.translation.y + 30.0 * time.delta_seconds();
        debug_info.output1 = format!("new_pos {:?}", new_pos);
        debug_info.output2 = format!("just fun {:?}", new_pos);
        player_transform.translation.y = new_pos;
    }

}
