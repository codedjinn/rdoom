use bevy::prelude::*;

pub fn setup_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
//   commands.spawn_bundle(TextBundle {
//     style: Style {
//         align_self: AlignSelf::FlexEnd,
//         position_type: PositionType::Absolute,
//         position: Rect {
//             top: Val::Px(5.0),
//             left: Val::Px(15.0),
//             ..Default::default()
//         },
//         ..Default::default()
//     },
//     // Use the `Text::with_section` constructor
//     text: Text::with_section(
//         // Accepts a `String` or any type that converts into a `String`, such as `&str`
//         "hello\nbevy!",
//         TextStyle {
//             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//             font_size: 18.0,
//             color: Color::WHITE,
//         },
//         // Note: You can use `Default::default()` in place of the `TextAlignment`
//         TextAlignment {
//             horizontal: HorizontalAlign::Center,
//             ..Default::default()
//         },
//     ),
//     ..Default::default()
//   });
}
