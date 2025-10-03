use bevy::{prelude::*, ui::Node};

pub fn load_hud(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            row_gap: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    ))
    .with_children(|parent| {
        for _ in 0..4 {
            parent.spawn((
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.8, 0.6)),
            ));
        }
    });
}
