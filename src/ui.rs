
pub use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_inspector_egui::bevy_egui::EguiContext;


use super::components::*;

pub fn health_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    entity_health: Query<(Entity, &Health)>
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("Health").show(context.get_mut(), |ui| {
            for (e, health) in entity_health.iter() {
                ui.heading(format!("{:#?}'s Health: {:#}/{:#}", e, health.hp, health.max_hp));
            }
        });
    }
}

// let text_vec = Vec::new();
// //text_vec.push(TextSection::new("Health: 100", TextStyle));
// commands.spawn(
//     (
//         TextBundle {
//             text: Text {
//                 sections: text_vec,
//                 ..default()
//             },  
//             ..default()
//         },
//         HealthText,
//     )
//     // TextBundle {
//     //     text: TextBundle {
//     //         sections: vec![TextSection {
//     //             value: "Health: 100".to_string(),
//     //             style: TextStyle {
//     //                 color: Color::rgb(1.0, 1.0, 1.0),
//     //                 font_size: Font::new(24.0),
//     //                 ..default()
//     //             },
//     //             ..default()
//     //         }],
//     //         alignment: TextAlignment {
//     //             horizontal: HorizontalAlign::Center,
//     //             vertical: VerticalAlign::Center,
//     //         },
//     //     },
//     //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
//     //    ..Default::default()
//     // }
// )
//   .insert(HealthText);
