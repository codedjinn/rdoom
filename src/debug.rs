use bevy::prelude::*;
pub struct DebugPlugin;

#[derive(Component)]
pub struct DebugDisplayInfo {
    pub output1: String,
    pub output2: String,
    pub output3: String,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_debug_info_text)
            .add_system(update_debug_info_text);
    }
}

pub fn update_debug_info_text(mut query: Query<(&mut Text, &DebugDisplayInfo)>) {
    for (mut text, debug_info) in query.iter_mut() {
        text.sections[0].value = format!("{}", debug_info.output1);        
        text.sections[1].value = format!("{}", debug_info.output2);
    }
}

pub fn startup_debug_info_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let debug_info = DebugDisplayInfo {
        output1: String::from(""),
        output2: String::from(""),
        output3: String::from(""),
    };

    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 12.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                        },

                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 12.0,
                            color: Color::rgb(1.0,  1.0,  1.0),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                
                ..Default::default()
            },
            ..Default::default()
        
        })
        .insert(debug_info);
}


// Use the `Text::with_section` constructor
// text: Text::with_section(
//     // Accepts a `String` or any type that converts into a `String`, such as `&str`
//     "!!DEBUG INFO!!",
//     TextStyle {
//         font: font,
//         font_size: 18.0,
//         color: Color::WHITE,
//     },
//     // Note: You can use `Default::default()` in place of the `TextAlignment`
//     TextAlignment {
//         horizontal: HorizontalAlign::Center,
//         ..Default::default()
//     },
// ),